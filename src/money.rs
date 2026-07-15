use std::fmt;
use std::marker::PhantomData;
use std::ops::{Add, Div, Mul, Sub};

// ============================================================
// Типы-алиасы для разных валют
// ============================================================

pub type Rubles = Money<currencies::Ruble>;
pub type Dollars = Money<currencies::Dollar>;
pub type Euros = Money<currencies::Euro>;
pub type Yuans = Money<currencies::Yuan>;

// ============================================================
// Основная структура Money
// ============================================================

/// Структура для представления денег в определенной валюте
/// Хранит значение в минимальных единицах (копейках, центах и т.д.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Money<C: Currency> {
    value: u32, // Используем u32 для большей вместимости
    _currency: PhantomData<C>,
}

impl<C: Currency> Money<C> {
    // ===== Константы =====

    /// Нулевая сумма
    pub const ZERO: Self = Self::from_minor(0);

    // ===== Конструкторы =====

    /// Создает деньги из минимальных единиц (копейки, центы)
    pub const fn from_minor(value: u32) -> Self {
        Self {
            value,
            _currency: PhantomData,
        }
    }

    /// Создает деньги из основных единиц (рубли, доллары)
    /// Использует коэффициент валюты для пересчета
    pub const fn from_major(value: u32) -> Self {
        Self {
            value: value * C::MINOR_UNITS,
            _currency: PhantomData,
        }
    }

    // ===== Геттеры =====

    /// Возвращает значение в минимальных единицах
    pub const fn minor(&self) -> u32 {
        self.value
    }

    /// Возвращает значение в основных единицах
    pub const fn major(&self) -> u32 {
        self.value / C::MINOR_UNITS
    }

    /// Возвращает символ валюты
    pub const fn symbol(&self) -> char {
        C::SYMBOL
    }

    /// Возвращает название валюты
    pub const fn currency_name(&self) -> &'static str {
        C::NAME
    }

    // ===== Проверки =====

    /// Проверяет, является ли сумма нулевой
    pub const fn is_zero(&self) -> bool {
        self.value == 0
    }

    // ===== Арифметические операции =====

    /// Сложение денег
    pub const fn add_money(self, rhs: Self) -> Self {
        Self::from_minor(self.value + rhs.value)
    }

    /// Вычитание денег
    pub const fn sub_money(self, rhs: Self) -> Self {
        Self::from_minor(self.value - rhs.value)
    }

    /// Умножение на целое число
    pub const fn mul_u32(self, value: u32) -> Self {
        Self::from_minor(self.value * value)
    }

    /// Деление на целое число
    pub const fn div_u32(self, value: u32) -> Self {
        Self::from_minor(self.value / value)
    }
}

// ============================================================
// Реализация трейтов для удобной работы
// ============================================================

impl<C: Currency> Add for Money<C> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.add_money(rhs)
    }
}

impl<C: Currency> Sub for Money<C> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.sub_money(rhs)
    }
}

impl<C: Currency> Mul<u32> for Money<C> {
    type Output = Self;

    fn mul(self, rhs: u32) -> Self::Output {
        self.mul_u32(rhs)
    }
}

impl<C: Currency> Mul<Money<C>> for u32 {
    type Output = Money<C>;

    fn mul(self, rhs: Money<C>) -> Self::Output {
        rhs.mul_u32(self)
    }
}

impl<C: Currency> Div<u32> for Money<C> {
    type Output = Self;

    fn div(self, rhs: u32) -> Self::Output {
        self.div_u32(rhs)
    }
}

impl<C: Currency> Default for Money<C> {
    fn default() -> Self {
        Self::ZERO
    }
}

impl<C: Currency> fmt::Display for Money<C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let major = self.value / C::MINOR_UNITS;
        let minor = self.value % C::MINOR_UNITS;
        let decimals = C::DECIMAL_PLACES;

        if decimals == 0 {
            write!(f, "{}{}", C::SYMBOL, major)
        } else {
            write!(
                f,
                "{}{}.{:0width$}",
                C::SYMBOL,
                major,
                minor,
                width = decimals as usize
            )
        }
    }
}

// ============================================================
// Трейт Currency
// ============================================================

/// Трейт для определения валюты
pub trait Currency: Sized + Send + Sync + 'static {
    /// Символ валюты
    const SYMBOL: char;

    /// Название валюты
    const NAME: &'static str;

    /// Количество минимальных единиц в одной основной единице
    /// Например: 100 копеек = 1 рубль, 100 центов = 1 доллар
    const MINOR_UNITS: u32;

    /// Количество знаков после запятой для отображения
    const DECIMAL_PLACES: u8;
}

// ============================================================
// Реализации для конкретных валют
// ============================================================

pub mod currencies {
    use super::Currency;

    pub struct Ruble;

    impl Currency for Ruble {
        const SYMBOL: char = '₽';
        const NAME: &'static str = "Russian Ruble";
        const MINOR_UNITS: u32 = 100;
        const DECIMAL_PLACES: u8 = 2;
    }

    pub struct Dollar;

    impl Currency for Dollar {
        const SYMBOL: char = '$';
        const NAME: &'static str = "US Dollar";
        const MINOR_UNITS: u32 = 100;
        const DECIMAL_PLACES: u8 = 2;
    }

    pub struct Euro;

    impl Currency for Euro {
        const SYMBOL: char = '€';
        const NAME: &'static str = "Euro";
        const MINOR_UNITS: u32 = 100;
        const DECIMAL_PLACES: u8 = 2;
    }

    pub struct Yuan;

    impl Currency for Yuan {
        const SYMBOL: char = '¥';
        const NAME: &'static str = "Chinese Yuan";
        const MINOR_UNITS: u32 = 100;
        const DECIMAL_PLACES: u8 = 2;
    }
}
