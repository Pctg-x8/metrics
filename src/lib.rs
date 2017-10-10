//! Metrics Definition

#[macro_use]
extern crate metrics_derives;

use std::ops::*;

/// 2D サイズ(整数)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Coordinate2)] #[repr(C)] pub struct Size2(pub i32, pub i32);
/// 2D サイズ(整数、符号なし)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Coordinate2)] #[repr(C)] pub struct Size2U(pub u32, pub u32);
/// 2D サイズ(実数)
#[derive(Clone, Copy, Debug, PartialEq, Coordinate2)] #[repr(C)] pub struct Size2F(pub f32, pub f32);
/// 2D 点(整数)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Coordinate2)] #[repr(C)] pub struct Point2(pub i32, pub i32);
/// 2D 点(整数、符号なし)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Coordinate2)] #[repr(C)] pub struct Point2U(pub u32, pub u32);
/// 2D 点(実数)
#[derive(Clone, Copy, Debug, PartialEq, Coordinate2)] #[repr(C)] pub struct Point2F(pub f32, pub f32);
/// 2D 点(倍精度実数)
#[derive(Clone, Debug, PartialEq, Coordinate2)] #[repr(C)] pub struct Point2LF(pub f64, pub f64);
/// 2D ベクトル
#[derive(Clone, Copy, Debug, PartialEq, Coordinate2)] #[repr(C)] pub struct Vector2(pub f32, pub f32);
/// 2D 矩形(整数)
#[derive(Clone, Debug, PartialEq, Eq)] #[repr(C)] pub struct Rect2(pub i32, pub i32, pub i32, pub i32);
/// 2D 矩形(整数、符号なし)
#[derive(Clone, Debug, PartialEq, Eq)] #[repr(C)] pub struct Rect2U(pub u32, pub u32, pub u32, pub u32);
/// 2D 矩形(実数)
#[derive(Clone, Debug, PartialEq)] #[repr(C)] pub struct Rect2F(pub f32, pub f32, pub f32, pub f32);

// 相互変換
impl Into<Size2U> for Size2  { fn into(self) -> Size2U { Size2U(self.0 as _, self.1 as _) } }
impl Into<Size2F> for Size2  { fn into(self) -> Size2F { Size2F(self.0 as _, self.1 as _) } }
impl Into<Size2>  for Size2U { fn into(self) -> Size2  { Size2 (self.0 as _, self.1 as _) } }
impl Into<Size2F> for Size2U { fn into(self) -> Size2F { Size2F(self.0 as _, self.1 as _) } }
impl Into<Size2U> for Size2F { fn into(self) -> Size2U { Size2U(self.0 as _, self.1 as _) } }
impl Into<Size2>  for Size2F { fn into(self) -> Size2  { Size2 (self.0 as _, self.1 as _) } }
impl Into<Point2U>  for Point2   { fn into(self) -> Point2U  { Point2U (self.0 as _, self.1 as _) } }
impl Into<Point2F>  for Point2   { fn into(self) -> Point2F  { Point2F (self.0 as _, self.1 as _) } }
impl Into<Point2LF> for Point2   { fn into(self) -> Point2LF { Point2LF(self.0 as _, self.1 as _) } }
impl Into<Point2>   for Point2U  { fn into(self) -> Point2   { Point2  (self.0 as _, self.1 as _) } }
impl Into<Point2F>  for Point2U  { fn into(self) -> Point2F  { Point2F (self.0 as _, self.1 as _) } }
impl Into<Point2LF> for Point2U  { fn into(self) -> Point2LF { Point2LF(self.0 as _, self.1 as _) } }
impl Into<Point2U>  for Point2F  { fn into(self) -> Point2U  { Point2U (self.0 as _, self.1 as _) } }
impl Into<Point2>   for Point2F  { fn into(self) -> Point2   { Point2  (self.0 as _, self.1 as _) } }
impl Into<Point2LF> for Point2F  { fn into(self) -> Point2LF { Point2LF(self.0 as _, self.1 as _) } }
impl Into<Point2U>  for Point2LF { fn into(self) -> Point2U  { Point2U (self.0 as _, self.1 as _) } }
impl Into<Point2>   for Point2LF { fn into(self) -> Point2   { Point2  (self.0 as _, self.1 as _) } }
impl Into<Point2F>  for Point2LF { fn into(self) -> Point2F  { Point2F (self.0 as _, self.1 as _) } }
impl<'r> Into<Rect2U> for &'r Rect2  { fn into(self) -> Rect2U { Rect2U(self.0 as _, self.1 as _, self.2 as _, self.3 as _) } }
impl<'r> Into<Rect2F> for &'r Rect2  { fn into(self) -> Rect2F { Rect2F(self.0 as _, self.1 as _, self.2 as _, self.3 as _) } }
impl<'r> Into<Rect2>  for &'r Rect2U { fn into(self) -> Rect2  { Rect2 (self.0 as _, self.1 as _, self.2 as _, self.3 as _) } }
impl<'r> Into<Rect2F> for &'r Rect2U { fn into(self) -> Rect2F { Rect2F(self.0 as _, self.1 as _, self.2 as _, self.3 as _) } }
impl<'r> Into<Rect2U> for &'r Rect2F { fn into(self) -> Rect2U { Rect2U(self.0 as _, self.1 as _, self.2 as _, self.3 as _) } }
impl<'r> Into<Rect2>  for &'r Rect2F { fn into(self) -> Rect2  { Rect2 (self.0 as _, self.1 as _, self.2 as _, self.3 as _) } }

/// 2D 座標系
/// # Examples
/// ```
/// # use metrics::*;
/// let p2 = Point2(8, 5);
/// 
/// assert_eq!(p2.x(), 8);
/// assert_eq!(p2.y(), 5);
/// ```
pub trait Coordinate2 : Sized
{
    /// 要素の型
    type Element: Add<Output = Self::Element> + Sub<Output = Self::Element>;
    /// x座標
    fn x(&self) -> Self::Element;
    /// y座標
    fn y(&self) -> Self::Element;
    /// 生成
    fn new(x: Self::Element, y: Self::Element) -> Self;
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
pub trait Size : Coordinate2 + Copy
{
    /// 幅
    fn width(self) -> Self::Element { self.x() }
    /// 高さ
    fn height(self) -> Self::Element { self.y() }

    /// アスペクト比(縦/横)を計算する
    fn aspect_w(self) -> f32 where Self::Element: ScalarConvertible<f32> { self.height()._as() / self.width()._as() }
    /// アスペクト比(横/縦)を計算する
    fn aspect_h(self) -> f32 where Self::Element: ScalarConvertible<f32> { self.width()._as() / self.height()._as() }
    /// 縮める
    fn shrink(self, x: Self::Element, y: Self::Element) -> Self { Self::new(self.width() - x, self.height() - y) }
    /// 伸ばす
    fn expand(self, x: Self::Element, y: Self::Element) -> Self { Self::new(self.width() + x, self.height() + y) }
}
impl Size for Size2 {}
impl Size for Size2U {}
impl Size for Size2F {}

/// 2D 矩形
pub trait Rect2T : Sized
{
    /// 要素の型
    type Element: ScalarConvertible<f32> + Add<Output = Self::Element> + Sub<Output = Self::Element> + Copy;
    /// サイズの型
    type SizeT: Size<Element = Self::Element>;
    /// オフセットの型
    type OffsetT: Coordinate2<Element = Self::Element>;

    /// 生成
    fn new(left: Self::Element, top: Self::Element, right: Self::Element, bottom: Self::Element) -> Self;
    /// オフセットとサイズから生成
    fn from_offset_and_size<O: Into<Self::OffsetT> + Copy, S: Into<Self::SizeT> + Copy>(offs: O, size: S) -> Self
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
    fn size(&self) -> Self::SizeT { Self::SizeT::new(self.right() - self.left(), self.bottom() - self.top()) }
    /// オフセット
    fn offset(&self) -> Self::OffsetT { Self::OffsetT::new(self.left(), self.top()) }

    /// 拡張
    fn extend(&self, amount: Self::Element) -> Self { Self::new(self.left() - amount, self.top() - amount, self.right() + amount, self.bottom() + amount) }
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
impl<T: Coordinate2 + Copy> Mul<T> for Vector2 where T::Element: ScalarConvertible<f32>
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
/// 保証済みなtransmute
pub fn transmute_safe<T: MarkForSameBits<U>, U>(input: &T) -> &U { unsafe { std::mem::transmute(input) } }
/// CoerceSameBitsがしっかり定義してあれば配列も高速にへんかんできる
pub fn transmute_array<T: MarkForSameBits<U>, U>(input: &[T]) -> &[U] { unsafe { std::mem::transmute(input) } }
unsafe impl MarkForSameBits<Point2> for Size2 {}
unsafe impl MarkForSameBits<Size2> for Point2 {}
unsafe impl MarkForSameBits<Point2U> for Size2U {}
unsafe impl MarkForSameBits<Size2U> for Point2U {}
unsafe impl MarkForSameBits<Point2F> for Size2F {}
unsafe impl MarkForSameBits<Size2F> for Point2F {}
unsafe impl MarkForSameBits<Point2F> for Vector2 {}
unsafe impl MarkForSameBits<Size2F> for Vector2 {}
unsafe impl MarkForSameBits<Vector2> for Point2F {}
unsafe impl MarkForSameBits<Vector2> for Size2F {}

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
#[cfg(windows)] unsafe impl MarkForSameBits<D2D1_SIZE_U> for Size2U {}
#[cfg(windows)] unsafe impl MarkForSameBits<D2D1_SIZE_F> for Size2F {}
#[cfg(windows)] unsafe impl MarkForSameBits<D2D1_POINT_2U> for Point2U {}
#[cfg(windows)] unsafe impl MarkForSameBits<D2D1_POINT_2F> for Point2F {}
#[cfg(windows)] unsafe impl MarkForSameBits<D2D1_RECT_U> for Rect2U {}
#[cfg(windows)] unsafe impl MarkForSameBits<D2D1_RECT_F> for Rect2F {}
#[cfg(windows)] unsafe impl MarkForSameBits<POINT> for Point2 {}
#[cfg(windows)] unsafe impl MarkForSameBits<SIZE> for Size2 {}
#[cfg(windows)] unsafe impl MarkForSameBits<RECT> for Rect2 {}
#[cfg(windows)] impl<'p> From<&'p D2D1_SIZE_U> for Size2 { fn from(other: &'p D2D1_SIZE_U) -> Self { unsafe { std::mem::transmute_copy(other) } } }
#[cfg(windows)] impl<'p> From<&'p D2D1_SIZE_F> for Size2F { fn from(other: &'p D2D1_SIZE_F) -> Self { unsafe { std::mem::transmute_copy(other) } } }
#[cfg(windows)] impl<'p> From<&'p D2D1_POINT_2U> for Point2 { fn from(other: &'p D2D1_POINT_2U) -> Self { unsafe { std::mem::transmute_copy(other) } } }
#[cfg(windows)] impl<'p> From<&'p D2D1_POINT_2F> for Point2F { fn from(other: &'p D2D1_POINT_2F) -> Self { unsafe { std::mem::transmute_copy(other) } } }
#[cfg(windows)] impl<'p> From<&'p D2D1_POINT_2F> for Vector2 { fn from(other: &'p D2D1_POINT_2F) -> Self { unsafe { std::mem::transmute_copy(other) } } }
#[cfg(windows)] impl<'p> From<&'p D2D1_RECT_F> for Rect2F { fn from(other: &'p D2D1_RECT_F) -> Self { unsafe { std::mem::transmute_copy(other) } } }
#[cfg(windows)] impl<'p> From<&'p POINT> for Point2 { fn from(other: &'p POINT) -> Self { Point2(other.x, other.y) } }
#[cfg(windows)] impl<'p> From<&'p SIZE> for Size2 { fn from(other: &'p SIZE) -> Self { Size2(other.cx, other.cy) } }
#[cfg(windows)] impl<'p> From<&'p RECT> for Rect2 { fn from(other: &'p RECT) -> Self { Rect2(other.left, other.top, other.right, other.bottom) } }
#[cfg(windows)] impl<'p> Into<POINT> for &'p Point2 { fn into(self) -> POINT { POINT { x: self.0 as _, y: self.1 as _ } } }
#[cfg(windows)] impl<'p> Into<SIZE> for &'p Size2 { fn into(self) -> SIZE { SIZE { cx: self.0 as _, cy: self.1 as _ } } }
#[cfg(windows)] impl<'p> Into<RECT> for &'p Rect2 { fn into(self) -> RECT { RECT { left: self.0 as _, top: self.1 as _, right: self.2 as _, bottom: self.3 as _ } } }

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
