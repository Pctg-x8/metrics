//! Metrics Definition

#[macro_use]
extern crate metrics_derives;

use std::ops::*;

/// 2D サイズ(整数)
#[derive(Clone, Debug, PartialEq, Coordinate2, Copy, Eq)]
#[repr(C)] pub struct Size2(pub i32, pub i32);
/// 2D サイズ(整数、符号なし)
#[derive(Clone, Debug, PartialEq, Coordinate2, Copy, Eq)]
#[repr(C)] pub struct Size2U(pub u32, pub u32);
/// 2D サイズ(実数)
#[derive(Clone, Debug, PartialEq, Coordinate2, Copy)]
#[repr(C)] pub struct Size2F(pub f32, pub f32);
/// 2D 点(整数)
#[derive(Clone, Debug, PartialEq, Coordinate2, Copy, Eq)]
#[repr(C)] pub struct Point2(pub i32, pub i32);
/// 2D 点(整数、符号なし)
#[derive(Clone, Debug, PartialEq, Coordinate2, Copy, Eq)]
#[repr(C)] pub struct Point2U(pub u32, pub u32);
/// 2D 点(実数)
#[derive(Clone, Debug, PartialEq, Coordinate2, Copy)]
#[repr(C)] pub struct Point2F(pub f32, pub f32);
/// 2D 点(倍精度実数)
#[derive(Clone, Debug, PartialEq, Coordinate2)]
#[repr(C)] pub struct Point2LF(pub f64, pub f64);
/// 2D ベクトル
#[derive(Clone, Debug, PartialEq, Coordinate2, Copy)]
#[repr(C)] pub struct Vector2(pub f32, pub f32);
/// 2D 矩形(整数)
#[derive(Clone, Debug, PartialEq, Eq)]
#[repr(C)] pub struct Rect2(pub i32, pub i32, pub i32, pub i32);
/// 2D 矩形(整数、符号なし)
#[derive(Clone, Debug, PartialEq, Eq)]
#[repr(C)] pub struct Rect2U(pub u32, pub u32, pub u32, pub u32);
/// 2D 矩形(実数)
#[derive(Clone, Debug, PartialEq)]
#[repr(C)] pub struct Rect2F(pub f32, pub f32, pub f32, pub f32);

// 相互変換
macro_rules! IntoByCasting
{
    (2 $org: ty => $($to: ty),+) =>
    {
        $(impl From<$org> for $to { fn from(v: $org) -> Self { Self::new(v.0 as _, v.1 as _) } })+
    };
    (4 $org: ty => $($to: ty),+) =>
    {
        $(impl From<$org> for $to { fn from(v: $org) -> Self { Self::new(v.0 as _, v.1 as _, v.2 as _, v.3 as _) } })+
    };
}
IntoByCasting!(2 Size2 => Size2U, Size2F);
IntoByCasting!(2 Size2U => Size2, Size2F);
IntoByCasting!(2 Size2F => Size2, Size2U);
IntoByCasting!(2 Point2 => Point2U, Point2F, Point2LF);
IntoByCasting!(2 Point2U => Point2, Point2F, Point2LF);
IntoByCasting!(2 Point2F => Point2U, Point2, Point2LF);
IntoByCasting!(2 Point2LF => Point2U, Point2F, Point2);
IntoByCasting!(4 Rect2 => Rect2U, Rect2F);
IntoByCasting!(4 Rect2U => Rect2, Rect2F);
IntoByCasting!(4 Rect2F => Rect2U, Rect2);

/// 2D 座標系
/// # Examples
/// ```
/// # use metrics::*;
/// let p2 = Point2(8, 5);
/// 
/// assert_eq!(p2.x(), 8);
/// assert_eq!(p2.y(), 5);
/// ```
pub trait Coordinate2
{
    /// 要素の型
    type Element;
    /// x座標
    fn x(&self) -> Self::Element;
    /// y座標
    fn y(&self) -> Self::Element;
    /// 生成
    fn new(x: Self::Element, y: Self::Element) -> Self where Self: Sized;
}
/// 2D サイズ
/// # Examples
/// ```
/// # use metrics::*;
/// let s2 = Size2(8, 6);
/// 
/// assert_eq!(s2.width(), s2.x());
/// assert_eq!(s2.height(), s2.y());
/// assert_eq!(s2.aspect_w(), 6.0 / 8.0);
/// assert_eq!(s2.aspect_h(), 8.0 / 6.0);
/// assert_eq!(s2.expand(2, 3), Size2(8 + 2, 6 + 3));
/// assert_eq!(s2.shrink(2, 3), Size2(8 - 2, 6 - 3));
/// ```
pub trait Size : Coordinate2
{
    /// 幅
    fn width(&self) -> Self::Element { self.x() }
    /// 高さ
    fn height(&self) -> Self::Element { self.y() }

    /// アスペクト比(縦/横)を計算する
    fn aspect_w(&self) -> f32 where Self::Element: ScalarConvertible<f32> { self.height()._as() / self.width()._as() }
    /// アスペクト比(横/縦)を計算する
    fn aspect_h(&self) -> f32 where Self::Element: ScalarConvertible<f32> { self.width()._as() / self.height()._as() }
    /// 縮める
    fn shrink(&self, x: Self::Element, y: Self::Element) -> Self where Self: Sized, Self::Element: Sub<Self::Element, Output = Self::Element>
    {
        Self::new(self.width() - x, self.height() - y)
    }
    /// 伸ばす
    fn expand(&self, x: Self::Element, y: Self::Element) -> Self where Self: Sized, Self::Element: Add<Self::Element, Output = Self::Element>
    {
        Self::new(self.width() + x, self.height() + y)
    }
}
impl Size for Size2 {}
impl Size for Size2U {}
impl Size for Size2F {}

/// 2D 矩形
pub trait Rect2T
{
    /// 要素の型
    type Element;
    /// サイズの型
    type SizeT: Size<Element = Self::Element>;
    /// オフセットの型
    type OffsetT: Coordinate2<Element = Self::Element>;

    /// 生成
    fn new(left: Self::Element, top: Self::Element, right: Self::Element, bottom: Self::Element) -> Self where Self: Sized;
    /// オフセットとサイズから生成
    fn from_offset_and_size<O: Into<Self::OffsetT> + Copy, S: Into<Self::SizeT> + Copy>(offs: O, size: S) -> Self
        where Self: Sized, Self::Element: Add<Self::Element, Output = Self::Element>
    {
        Self::new(offs.into().x(), offs.into().y(), offs.into().x() + size.into().width(), offs.into().y() + size.into().height())
    }

    /// 左端
    fn left(&self) -> Self::Element;
    /// 上橋
    fn top(&self) -> Self::Element;
    /// 右端
    fn right(&self) -> Self::Element;
    /// 下橋
    fn bottom(&self) -> Self::Element;

    /// 横位置
    fn x(&self) -> Self::Element { self.left() }
    /// 縦位置
    fn y(&self) -> Self::Element { self.top() }
    /// サイズ
    fn size(&self) -> Self::SizeT where Self::Element: Sub<Self::Element, Output = Self::Element>
    {
        Self::SizeT::new(self.right() - self.left(), self.bottom() - self.top())
    }
    /// オフセット
    fn offset(&self) -> Self::OffsetT { Self::OffsetT::new(self.left(), self.top()) }

    /// 左上
    fn lt(&self) -> Self::OffsetT { self.offset() }
    /// 右上
    fn rt(&self) -> Self::OffsetT { Self::OffsetT::new(self.right(), self.top()) }
    /// 左下
    fn lb(&self) -> Self::OffsetT { Self::OffsetT::new(self.left(), self.bottom()) }
    /// 右下
    fn rb(&self) -> Self::OffsetT { Self::OffsetT::new(self.right(), self.bottom()) }

    /// 拡張
    fn extend(&self, amount: Self::Element) -> Self where Self: Sized, Self::Element: Sub<Self::Element, Output = Self::Element> + Add<Self::Element, Output = Self::Element> + Clone
    {
        Self::new(self.left() - amount.clone(), self.top() - amount.clone(), self.right() + amount.clone(), self.bottom() + amount)
    }
}
impl Rect2T for Rect2
{
    type Element = i32; type SizeT = Size2; type OffsetT = Point2;

    fn new(left: i32, top: i32, right: i32, bottom: i32) -> Self { Rect2(left, top, right, bottom) }
    fn left(&self) -> i32 { self.0 }
    fn top(&self) -> i32 { self.1 }
    fn right(&self) -> i32 { self.2 }
    fn bottom(&self) -> i32 { self.3 }
}
impl Rect2T for Rect2U
{
    type Element = u32; type SizeT = Size2U; type OffsetT = Point2U;

    fn new(left: u32, top: u32, right: u32, bottom: u32) -> Self { Rect2U(left, top, right, bottom) }
    fn left(&self) -> u32 { self.0 }
    fn top(&self) -> u32 { self.1 }
    fn right(&self) -> u32 { self.2 }
    fn bottom(&self) -> u32 { self.3 }
}
impl Rect2T for Rect2F
{
    type Element = f32; type SizeT = Size2F; type OffsetT = Point2F;

    fn new(left: f32, top: f32, right: f32, bottom: f32) -> Self { Rect2F(left, top, right, bottom) }
    fn left(&self) -> f32 { self.0 }
    fn top(&self) -> f32 { self.1 }
    fn right(&self) -> f32 { self.2 }
    fn bottom(&self) -> f32 { self.3 }
}

impl Vector2
{
    /// 極座標系で生成
    pub fn polar(r: f32, d: f32) -> Self
    {
        let (s, c) = r.sin_cos();
        Vector2(c * d, s * d)
    }
}

// スカラー演算定義
macro_rules! ScalarOps
{
    (for4 <$e: ty> $($t: ident),*) =>
    {
        $(
            impl Mul<$e> for $t { type Output = Self; fn mul(self, other: $e) -> Self { $t(self.0 * other, self.1 * other, self.2 * other, self.3 * other) } }
            impl Div<$e> for $t { type Output = Self; fn div(self, other: $e) -> Self { $t(self.0 / other, self.1 / other, self.2 / other, self.3 / other) } }
        )*
    }
}
ScalarOps!(for4<f32> Rect2F);
ScalarOps!(for4<u32> Rect2U);
ScalarOps!(for4<i32> Rect2);
impl<T: Coordinate2> Mul<T> for Vector2 where T::Element: ScalarConvertible<f32>
{
    type Output = Vector2;
    fn mul(self, other: T) -> Vector2 { Vector2(self.0 * other.x()._as(), self.1 * other.y()._as()) }
}

// 表示データ生成
macro_rules! Displayable
{
    (for $($t: ty),*) => 
    {
        $(impl std::fmt::Display for $t
        {
            fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result
            {
                write!(fmt, "{}({}, {})", stringify!($t), self.0, self.1)
            }
        })*
    };
    (forRect $($t: ty),*) => 
    {
        $(impl std::fmt::Display for $t
        {
            fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result
            {
                write!(fmt, "{}[({}, {})-({}, {})]", stringify!($t), self.0, self.1, self.2, self.3)
            }
        })*
    };
}
Displayable!(for Size2, Size2U, Size2F, Point2, Point2U, Point2F, Point2LF, Vector2);
Displayable!(forRect Rect2, Rect2U, Rect2F);

/// 同じビット配置であることを保証する
pub unsafe trait MarkForSameBits<T> {}
macro_rules! MarkSameLayout
{
    ($s: ty = $d: ty) =>
    {
        unsafe impl MarkForSameBits<$s> for $d {}
        unsafe impl MarkForSameBits<$d> for $s {}
        impl AsRef<$s> for $d { fn as_ref(&self) -> &$s { transmute_safe(self) } }
        impl AsRef<$d> for $s { fn as_ref(&self) -> &$d { transmute_safe(self) } }
    };
}
/// 保証済みなtransmute
pub fn transmute_safe<T: MarkForSameBits<U>, U>(input: &T) -> &U { unsafe { std::mem::transmute(input) } }
/// CoerceSameBitsがしっかり定義してあれば配列も高速にへんかんできる
pub fn transmute_array<T: MarkForSameBits<U>, U>(input: &[T]) -> &[U] { unsafe { std::mem::transmute(input) } }
MarkSameLayout!(Point2  = Size2);
MarkSameLayout!(Point2U = Size2U);
MarkSameLayout!(Point2F = Size2F);
MarkSameLayout!(Vector2 = Point2F);
MarkSameLayout!(Vector2 = Size2F);

/// 整数 <-> 実数間の変換を定義する
pub trait ScalarConvertible<T> { fn _as(self) -> T; }
macro_rules! DefScalarConv
{
    ($s: ty => $($d: ty),*) =>
    {
        $(impl ScalarConvertible<$d> for $s { fn _as(self) -> $d { self as _ } })*
    }
}
DefScalarConv!(u8  => u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, f32, f64);
DefScalarConv!(i8  => u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, f32, f64);
DefScalarConv!(u16 => u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, f32, f64);
DefScalarConv!(i16 => u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, f32, f64);
DefScalarConv!(u32 => u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, f32, f64);
DefScalarConv!(i32 => u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, f32, f64);
DefScalarConv!(u64 => u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, f32, f64);
DefScalarConv!(i64 => u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, f32, f64);
DefScalarConv!(usize => u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, f32, f64);
DefScalarConv!(isize => u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, f32, f64);
DefScalarConv!(f32 => u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, f32, f64);
DefScalarConv!(f64 => u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, f32, f64);

// Windows/Direct2D向けの変換
#[cfg(windows)] extern crate winapi;
#[cfg(windows)] use winapi::shared::windef::{RECT, POINT, SIZE};
#[cfg(windows)] use winapi::um::d2d1::*;
#[cfg(windows)] MarkSameLayout!(Size2U = D2D1_SIZE_U);
#[cfg(windows)] MarkSameLayout!(Size2F = D2D1_SIZE_F);
#[cfg(windows)] MarkSameLayout!(Point2U = D2D1_POINT_2U);
#[cfg(windows)] MarkSameLayout!(Point2F = D2D1_POINT_2F);
#[cfg(windows)] MarkSameLayout!(Rect2U = D2D1_RECT_U);
#[cfg(windows)] MarkSameLayout!(Rect2F = D2D1_RECT_F);
#[cfg(windows)] MarkSameLayout!(Point2 = POINT);
#[cfg(windows)] MarkSameLayout!(Size2 = SIZE);
#[cfg(windows)] MarkSameLayout!(Rect2 = RECT);
#[cfg(windows)] impl From<D2D1_SIZE_U> for Size2 { fn from(other: D2D1_SIZE_U) -> Self { unsafe { std::mem::transmute(other) } } }
#[cfg(windows)] impl From<D2D1_SIZE_U> for Size2U { fn from(other: D2D1_SIZE_U) -> Self { unsafe { std::mem::transmute(other) } } }
#[cfg(windows)] impl From<D2D1_SIZE_F> for Size2F { fn from(other: D2D1_SIZE_F) -> Self { unsafe { std::mem::transmute(other) } } }
#[cfg(windows)] impl From<D2D1_POINT_2U> for Point2 { fn from(other: D2D1_POINT_2U) -> Self { unsafe { std::mem::transmute(other) } } }
#[cfg(windows)] impl From<D2D1_POINT_2F> for Point2F { fn from(other: D2D1_POINT_2F) -> Self { unsafe { std::mem::transmute(other) } } }
#[cfg(windows)] impl From<D2D1_POINT_2F> for Vector2 { fn from(other: D2D1_POINT_2F) -> Self { unsafe { std::mem::transmute(other) } } }
#[cfg(windows)] impl From<D2D1_RECT_F> for Rect2F { fn from(other: D2D1_RECT_F) -> Self { unsafe { std::mem::transmute(other) } } }
#[cfg(windows)] impl From<POINT> for Point2 { fn from(other: POINT) -> Self { Point2(other.x, other.y) } }
#[cfg(windows)] impl From<SIZE> for Size2 { fn from(other: SIZE) -> Self { Size2(other.cx, other.cy) } }
#[cfg(windows)] impl From<RECT> for Rect2 { fn from(other: RECT) -> Self { Rect2(other.left, other.top, other.right, other.bottom) } }

#[cfg(test)]
mod test
{
    use super::*;

    #[test] fn scalar_ops()
    {
        assert_eq!(Point2F(0.0, 3.0) * 4.0, Point2F(0.0, 12.0));
        assert_eq!(Vector2(3.0, 2.0) * -2.5, Vector2(3.0 * -2.5, 2.0 * -2.5));
        assert_eq!(Rect2U(2, 4, 6, 8) / 2, Rect2U(1, 2, 3, 4));
    }
}
