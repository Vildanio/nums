use std::cmp::Ordering;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/// Структура для представления объема
/// Хранится в кубических сантиметрах (см³) как наименьшая единица
/// 1 см³ = 1 миллилитр (мл)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Volume {
    cm3: u32, // Кубические сантиметры (миллилитры)
}

impl Volume {
    // ========================================
    // Константы
    // ========================================

    /// Нулевой объем
    pub const ZERO: Self = Self { cm3: 0 };

    // ========================================
    // Конструкторы
    // ========================================

    /// Создает объем из кубических сантиметров (миллилитров)
    pub const fn from_cm3(value: u32) -> Self {
        Self { cm3: value }
    }

    /// Создает объем из литров
    pub const fn from_liters(value: u32) -> Self {
        Self { cm3: value * 1000 }
    }

    /// Создает объем из кубических метров
    pub const fn from_m3(value: u32) -> Self {
        Self {
            cm3: value * 1_000_000,
        }
    }

    /// Создает объем из кубических дециметров (эквивалентно литрам)
    pub const fn from_dm3(value: u32) -> Self {
        Self { cm3: value * 1000 }
    }

    /// Создает объем из миллилитров (эквивалентно см³)
    pub const fn from_ml(value: u32) -> Self {
        Self { cm3: value }
    }

    /// Создает объем из кубических миллиметров
    pub const fn from_mm3(value: u32) -> Self {
        Self { cm3: value / 1000 }
    }

    // ========================================
    // Геттеры
    // ========================================

    /// Возвращает объем в кубических сантиметрах (миллилитрах)
    pub const fn as_cm3(&self) -> u32 {
        self.cm3
    }

    /// Возвращает объем в миллилитрах
    pub const fn as_ml(&self) -> u32 {
        self.cm3
    }

    /// Возвращает объем в литрах (с округлением вниз)
    pub const fn as_liters(&self) -> u32 {
        self.cm3 / 1000
    }

    /// Возвращает объем в литрах и остаток в миллилитрах
    pub const fn as_liters_and_ml(&self) -> (u32, u32) {
        let liters = self.cm3 / 1000;
        let ml = self.cm3 % 1000;
        (liters, ml)
    }

    /// Возвращает объем в кубических метрах (с округлением вниз)
    pub const fn as_m3(&self) -> u32 {
        self.cm3 / 1_000_000
    }

    /// Возвращает объем в кубических метрах и остаток в литрах
    pub const fn as_m3_and_liters(&self) -> (u32, u32) {
        let m3 = self.cm3 / 1_000_000;
        let liters = (self.cm3 % 1_000_000) / 1000;
        (m3, liters)
    }

    /// Возвращает объем в кубических дециметрах (эквивалентно литрам)
    pub const fn as_dm3(&self) -> u32 {
        self.cm3 / 1000
    }

    /// Возвращает объем в кубических миллиметрах
    pub const fn as_mm3(&self) -> u32 {
        self.cm3 * 1000
    }

    // ========================================
    // Проверки
    // ========================================

    /// Проверяет, является ли объем нулевым
    pub const fn is_zero(&self) -> bool {
        self.cm3 == 0
    }

    /// Проверяет, является ли объем положительным
    pub const fn is_positive(&self) -> bool {
        self.cm3 > 0
    }

    // ========================================
    // Арифметические операции (const)
    // ========================================

    /// Сложение объемов
    pub const fn add_volume(&self, other: &Self) -> Self {
        Self::from_cm3(self.cm3 + other.cm3)
    }

    /// Вычитание объемов
    pub const fn sub_volume(&self, other: &Self) -> Self {
        Self::from_cm3(self.cm3 - other.cm3)
    }

    /// Умножение на целое число
    pub const fn mul_u32(&self, value: u32) -> Self {
        Self::from_cm3(self.cm3 * value)
    }

    /// Деление на целое число
    pub const fn div_u32(&self, value: u32) -> Self {
        Self::from_cm3(self.cm3 / value)
    }

    // ========================================
    // Полезные методы
    // ========================================

    /// Возвращает минимальный из двух объемов
    pub const fn min(&self, other: &Self) -> Self {
        if self.cm3 <= other.cm3 { *self } else { *other }
    }

    /// Возвращает максимальный из двух объемов
    pub const fn max(&self, other: &Self) -> Self {
        if self.cm3 >= other.cm3 { *self } else { *other }
    }

    /// Ограничивает объем диапазоном [min, max]
    pub const fn clamp(&self, min: &Self, max: &Self) -> Self {
        if self.cm3 < min.cm3 {
            *min
        } else if self.cm3 > max.cm3 {
            *max
        } else {
            *self
        }
    }
}

// ========================================
// Реализация трейтов
// ========================================

impl std::fmt::Display for Volume {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cm3 = self.cm3;

        if cm3 >= 1_000_000 {
            let m3 = cm3 / 1_000_000;
            let liters = (cm3 % 1_000_000) / 1000;
            let ml = cm3 % 1000;

            if liters == 0 && ml == 0 {
                write!(f, "{} м³", m3)
            } else if ml == 0 {
                write!(f, "{}.{:03} м³", m3, liters)
            } else {
                write!(f, "{}.{:03}{:03} м³", m3, liters, ml)
            }
        } else if cm3 >= 1000 {
            let liters = cm3 / 1000;
            let ml = cm3 % 1000;

            if ml == 0 {
                write!(f, "{} л", liters)
            } else {
                write!(f, "{}.{:03} л", liters, ml)
            }
        } else if cm3 == 0 {
            write!(f, "0 мл")
        } else {
            write!(f, "{} мл", cm3)
        }
    }
}

impl PartialOrd for Volume {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Volume {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cm3.cmp(&other.cm3)
    }
}

impl Add for Volume {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.add_volume(&rhs)
    }
}

impl AddAssign for Volume {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Volume {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.sub_volume(&rhs)
    }
}

impl SubAssign for Volume {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Mul<u32> for Volume {
    type Output = Self;

    fn mul(self, rhs: u32) -> Self::Output {
        self.mul_u32(rhs)
    }
}

impl Mul<Volume> for u32 {
    type Output = Volume;

    fn mul(self, rhs: Volume) -> Self::Output {
        rhs.mul_u32(self)
    }
}

impl MulAssign<u32> for Volume {
    fn mul_assign(&mut self, rhs: u32) {
        *self = *self * rhs;
    }
}

impl Div<u32> for Volume {
    type Output = Self;

    fn div(self, rhs: u32) -> Self::Output {
        self.div_u32(rhs)
    }
}

impl DivAssign<u32> for Volume {
    fn div_assign(&mut self, rhs: u32) {
        *self = *self / rhs;
    }
}

impl Default for Volume {
    fn default() -> Self {
        Self::ZERO
    }
}

impl From<u32> for Volume {
    fn from(cm3: u32) -> Self {
        Self { cm3 }
    }
}

impl From<Volume> for u32 {
    fn from(volume: Volume) -> Self {
        volume.cm3
    }
}
