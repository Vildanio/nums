use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

/// Структура для представления площади
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Area {
    square_meters: f64, // Храним в квадратных метрах как базовой единице
}

impl Area {
    /// Создает площадь из квадратных метров
    pub const fn from_square_meters(m2: f64) -> Self {
        Self { square_meters: m2 }
    }

    /// Создает площадь из квадратных километров
    pub const fn from_square_kilometers(km2: f64) -> Self {
        Self {
            square_meters: km2 * 1_000_000.0,
        }
    }

    /// Создает площадь из квадратных сантиметров
    pub const fn from_square_centimeters(cm2: f64) -> Self {
        Self {
            square_meters: cm2 / 10_000.0,
        }
    }

    /// Создает площадь из квадратных миллиметров
    pub const fn from_square_millimeters(mm2: f64) -> Self {
        Self {
            square_meters: mm2 / 1_000_000.0,
        }
    }

    /// Создает площадь из квадратных миль
    pub const fn from_square_miles(mi2: f64) -> Self {
        Self {
            square_meters: mi2 * 2_589_988.11,
        }
    }

    /// Создает площадь из акров
    pub const fn from_acres(acres: f64) -> Self {
        Self {
            square_meters: acres * 4046.85642,
        }
    }

    /// Создает площадь из гектаров
    pub const fn from_hectares(ha: f64) -> Self {
        Self {
            square_meters: ha * 10_000.0,
        }
    }

    /// Создает площадь из квадратных футов
    pub const fn from_square_feet(ft2: f64) -> Self {
        Self {
            square_meters: ft2 * 0.09290304,
        }
    }

    /// Создает площадь из квадратных ярдов
    pub const fn from_square_yards(yd2: f64) -> Self {
        Self {
            square_meters: yd2 * 0.83612736,
        }
    }

    /// Создает площадь из соток (ар)
    pub const fn from_ares(a: f64) -> Self {
        Self {
            square_meters: a * 100.0,
        }
    }

    // ===== Конвертеры в разные единицы =====

    /// Возвращает площадь в квадратных метрах
    pub const fn as_square_meters(&self) -> f64 {
        self.square_meters
    }

    /// Возвращает площадь в квадратных километрах
    pub const fn as_square_kilometers(&self) -> f64 {
        self.square_meters / 1_000_000.0
    }

    /// Возвращает площадь в квадратных сантиметрах
    pub const fn as_square_centimeters(&self) -> f64 {
        self.square_meters * 10_000.0
    }

    /// Возвращает площадь в квадратных миллиметрах
    pub const fn as_square_millimeters(&self) -> f64 {
        self.square_meters * 1_000_000.0
    }

    /// Возвращает площадь в гектарах
    pub const fn as_hectares(&self) -> f64 {
        self.square_meters / 10_000.0
    }

    /// Возвращает площадь в акрах
    pub const fn as_acres(&self) -> f64 {
        self.square_meters / 4046.85642
    }

    /// Возвращает площадь в квадратных милях
    pub const fn as_square_miles(&self) -> f64 {
        self.square_meters / 2_589_988.11
    }

    /// Возвращает площадь в квадратных футах
    pub const fn as_square_feet(&self) -> f64 {
        self.square_meters / 0.09290304
    }

    /// Возвращает площадь в квадратных ярдах
    pub const fn as_square_yards(&self) -> f64 {
        self.square_meters / 0.83612736
    }

    /// Возвращает площадь в сотках (ар)
    pub const fn as_ares(&self) -> f64 {
        self.square_meters / 100.0
    }

    // ===== Арифметические операции =====

    /// Сложение площадей
    pub const fn add(&self, other: &Self) -> Self {
        Self {
            square_meters: self.square_meters + other.square_meters,
        }
    }

    /// Вычитание площадей
    pub const fn sub(&self, other: &Self) -> Option<Self> {
        if self.square_meters >= other.square_meters {
            Some(Self {
                square_meters: self.square_meters - other.square_meters,
            })
        } else {
            None
        }
    }

    /// Умножение на число
    pub const fn mul(&self, n: f64) -> Self {
        Self {
            square_meters: self.square_meters * n,
        }
    }

    /// Деление на число
    pub const fn div(&self, n: f64) -> Option<Self> {
        if n != 0.0 {
            Some(Self {
                square_meters: self.square_meters / n,
            })
        } else {
            None
        }
    }

    // ===== Полезные методы =====

    /// Проверяет, является ли площадь нулевой
    pub const fn is_zero(&self) -> bool {
        self.square_meters.abs() < f64::EPSILON
    }

    /// Округляет до указанного количества знаков после запятой
    pub fn round(&self, decimals: u32) -> Self {
        let factor = 10_f64.powi(decimals as i32);
        Self {
            square_meters: (self.square_meters * factor).round() / factor,
        }
    }

    /// Возвращает абсолютное значение (для положительной площади)
    pub const fn abs(&self) -> Self {
        Self {
            square_meters: self.square_meters.abs(),
        }
    }

    /// Сравнивает две площади
    pub fn compare(&self, other: &Self) -> std::cmp::Ordering {
        self.square_meters
            .partial_cmp(&other.square_meters)
            .unwrap_or(std::cmp::Ordering::Equal)
    }
}

// ===== Реализация трейтов =====

impl std::fmt::Display for Area {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let m2 = self.square_meters;

        if m2 >= 1_000_000.0 {
            write!(f, "{:.2} км²", self.as_square_kilometers())
        } else if m2 >= 10000.0 {
            write!(f, "{:.2} га", self.as_hectares())
        } else if m2 >= 1.0 {
            write!(f, "{:.2} м²", m2)
        } else if m2 >= 0.0001 {
            write!(f, "{:.2} см²", self.as_square_centimeters())
        } else {
            write!(f, "{:.2} мм²", self.as_square_millimeters())
        }
    }
}

impl Add for Area {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            square_meters: self.square_meters + other.square_meters,
        }
    }
}

impl AddAssign for Area {
    fn add_assign(&mut self, other: Self) {
        self.square_meters += other.square_meters;
    }
}

impl Sub for Area {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            square_meters: self.square_meters - other.square_meters,
        }
    }
}

impl SubAssign for Area {
    fn sub_assign(&mut self, other: Self) {
        self.square_meters -= other.square_meters;
    }
}

impl Mul<f64> for Area {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Self {
            square_meters: self.square_meters * rhs,
        }
    }
}

impl Div<f64> for Area {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        Self {
            square_meters: self.square_meters / rhs,
        }
    }
}
