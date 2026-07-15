use std::cmp::Ordering;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/// Структура для представления длины (хранится в метрах, точность 1 метр)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Length {
    meters: u32, // Храним в метрах как целое число
}

impl Length {
    // ========================================
    // Конструкторы
    // ========================================

    /// Создает длину из метров
    pub const fn from_meters(m: u32) -> Self {
        Self { meters: m }
    }

    /// Создает длину из километров
    pub const fn from_kilometers(km: u32) -> Self {
        Self { meters: km * 1000 }
    }

    /// Создает длину из миль (1 миля = 1609.344 метра)
    pub fn from_miles(mi: u32) -> Self {
        // Используем более точный коэффициент: 1609344 / 1000
        Self {
            meters: (mi as f64 * 1609.344).round() as u32,
        }
    }

    /// Создает длину из футов (1 фут = 0.3048 метра)
    pub fn from_feet(ft: u32) -> Self {
        // Используем точный коэффициент: 3048 / 10000
        Self {
            meters: (ft as f64 * 0.3048).round() as u32,
        }
    }

    /// Создает длину из дюймов (1 дюйм = 0.0254 метра)
    pub fn from_inches(inch: u32) -> Self {
        // Используем точный коэффициент: 254 / 10000
        Self {
            meters: (inch as f64 * 0.0254).round() as u32,
        }
    }

    /// Создает длину из ярдов (1 ярд = 0.9144 метра)
    pub fn from_yards(yd: u32) -> Self {
        // Используем точный коэффициент: 9144 / 10000
        Self {
            meters: (yd as f64 * 0.9144).round() as u32,
        }
    }

    /// Нулевая длина
    pub const fn zero() -> Self {
        Self { meters: 0 }
    }

    // ========================================
    // Конвертеры
    // ========================================

    /// Возвращает длину в метрах
    pub const fn into_meters(&self) -> u32 {
        self.meters
    }

    /// Возвращает длину в километрах (целое число)
    pub const fn into_kilometers(&self) -> u32 {
        self.meters / 1000
    }

    /// Возвращает длину в километрах с остатком в метрах
    pub const fn into_kilometers_and_meters(&self) -> (u32, u32) {
        let km = self.meters / 1000;
        let m = self.meters % 1000;
        (km, m)
    }

    /// Возвращает длину в сантиметрах (приблизительно)
    pub fn into_centimeters(&self) -> u32 {
        self.meters * 100
    }

    /// Возвращает длину в миллиметрах (приблизительно)
    pub fn into_millimeters(&self) -> u32 {
        self.meters * 1000
    }

    /// Возвращает длину в милях (приблизительно)
    pub fn into_miles(&self) -> f64 {
        self.meters as f64 / 1609.344
    }

    // ========================================
    // Арифметические операции
    // ========================================

    /// Сложение длин
    pub const fn add(&self, other: &Self) -> Self {
        Self {
            meters: self.meters + other.meters,
        }
    }

    /// Вычитание длин
    pub const fn sub(&self, other: &Self) -> Option<Self> {
        if self.meters >= other.meters {
            Some(Self {
                meters: self.meters - other.meters,
            })
        } else {
            None
        }
    }

    /// Умножение на число
    pub const fn mul(&self, n: u32) -> Self {
        Self {
            meters: self.meters * n,
        }
    }

    /// Деление на число
    pub const fn div(&self, n: u32) -> Option<Self> {
        if n != 0 {
            Some(Self {
                meters: self.meters / n,
            })
        } else {
            None
        }
    }

    // ========================================
    // Полезные методы
    // ========================================

    /// Проверяет, является ли длина нулевой
    pub const fn is_zero(&self) -> bool {
        self.meters == 0
    }

    /// Проверяет, является ли длина положительной
    pub const fn is_positive(&self) -> bool {
        self.meters > 0
    }

    /// Проверяет, является ли длина четной (в метрах)
    pub const fn is_even(&self) -> bool {
        self.meters % 2 == 0
    }

    /// Проверяет, является ли длина нечетной (в метрах)
    pub const fn is_odd(&self) -> bool {
        self.meters % 2 != 0
    }

    /// Возвращает минимальную из двух длин
    pub const fn min(&self, other: &Self) -> Self {
        if self.meters <= other.meters {
            *self
        } else {
            *other
        }
    }

    /// Возвращает максимальную из двух длин
    pub const fn max(&self, other: &Self) -> Self {
        if self.meters >= other.meters {
            *self
        } else {
            *other
        }
    }

    /// Ограничивает длину диапазоном [min, max]
    pub const fn clamp(&self, min: &Self, max: &Self) -> Self {
        if self.meters < min.meters {
            *min
        } else if self.meters > max.meters {
            *max
        } else {
            *self
        }
    }
}

// ========================================
// Реализация трейтов
// ========================================

impl std::fmt::Display for Length {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let meters = self.meters;

        if meters >= 1000 {
            let km = meters / 1000;
            let m = meters % 1000;
            if m == 0 {
                write!(f, "{} км", km)
            } else {
                write!(f, "{}.{:03} км", km, m)
            }
        } else {
            write!(f, "{} м", meters)
        }
    }
}

impl PartialOrd for Length {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Length {
    fn cmp(&self, other: &Self) -> Ordering {
        self.meters.cmp(&other.meters)
    }
}

impl Add for Length {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            meters: self.meters + other.meters,
        }
    }
}

impl AddAssign for Length {
    fn add_assign(&mut self, other: Self) {
        self.meters += other.meters;
    }
}

impl Sub for Length {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            meters: self.meters - other.meters,
        }
    }
}

impl SubAssign for Length {
    fn sub_assign(&mut self, other: Self) {
        self.meters -= other.meters;
    }
}

impl Mul<u32> for Length {
    type Output = Self;
    fn mul(self, rhs: u32) -> Self {
        Self {
            meters: self.meters * rhs,
        }
    }
}

impl Mul<Length> for u32 {
    type Output = Length;
    fn mul(self, rhs: Length) -> Length {
        Length {
            meters: self * rhs.meters,
        }
    }
}

impl MulAssign<u32> for Length {
    fn mul_assign(&mut self, rhs: u32) {
        self.meters *= rhs;
    }
}

impl Div<u32> for Length {
    type Output = Self;
    fn div(self, rhs: u32) -> Self {
        Self {
            meters: self.meters / rhs,
        }
    }
}

impl DivAssign<u32> for Length {
    fn div_assign(&mut self, rhs: u32) {
        self.meters /= rhs;
    }
}

impl Default for Length {
    fn default() -> Self {
        Self { meters: 0 }
    }
}

impl From<u32> for Length {
    fn from(meters: u32) -> Self {
        Self { meters }
    }
}

impl From<Length> for u32 {
    fn from(length: Length) -> Self {
        length.meters
    }
}
