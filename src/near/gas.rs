use crate::near::constants::{ONE_GIGA_GAS, ONE_TERA_GAS};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NearGas {
    pub inner: u64,
}

impl std::fmt::Display for NearGas {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.inner == 0 {
            write!(f, "0 Gas")
        } else if self.inner >= ONE_TERA_GAS {
            write!(
                f,
                "{}.{:0>3} TeraGas",
                self.inner / ONE_TERA_GAS,
                self.inner / (ONE_TERA_GAS / 1000) % 1000
            )
        } else {
            write!(
                f,
                "{}.{:0>3} GigaGas",
                self.inner / ONE_GIGA_GAS,
                self.inner / (ONE_GIGA_GAS / 1000) % 1000
            )
        }
    }
}

impl std::str::FromStr for NearGas {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let num = s.trim().trim_end_matches(char::is_alphabetic).trim();
        let currency = s.trim().trim_start_matches(&num).trim().to_uppercase();
        let number = match currency.as_str() {
            "T" | "TGAS" | "TERAGAS" => NearGas::into_tera_gas(num)?,
            "GIGAGAS" | "GGAS" => NearGas::into_tera_gas(num)? / 1000,
            _ => return Err("Near Gas: incorrect currency value entered".to_string()),
        };
        Ok(NearGas { inner: number })
    }
}

impl From<u64> for NearGas {
    fn from(num: u64) -> Self {
        Self { inner: num }
    }
}

impl NearGas {
    fn into_tera_gas(num: &str) -> Result<u64, String> {
        let res_split: Vec<&str> = num.split('.').collect();
        match res_split.len() {
            2 => {
                let num_int_gas: u64 = res_split[0]
                    .parse::<u64>()
                    .map_err(|err| format!("Near Gas: {}", err))?
                    .checked_mul(10u64.pow(12))
                    .ok_or_else(|| "Near Gas: underflow or overflow happens")?;
                let len_fract = res_split[1].len() as u32;
                let num_fract_gas = if len_fract <= 12 {
                    res_split[1]
                        .parse::<u64>()
                        .map_err(|err| format!("Near Gas: {}", err))?
                        .checked_mul(10u64.pow(12 - res_split[1].len() as u32))
                        .ok_or_else(|| "Near Gas: underflow or overflow happens")?
                } else {
                    return Err("Near Gas: too large fractional part of a number".to_string());
                };
                Ok(num_int_gas
                    .checked_add(num_fract_gas)
                    .ok_or_else(|| "Near Gas: underflow or overflow happens")?)
            }
            1 => Ok(res_split[0]
                .parse::<u64>()
                .map_err(|err| format!("Near Gas: {}", err))?
                .checked_mul(10u64.pow(12))
                .ok_or_else(|| "Near Gas: underflow or overflow happens")?),
            _=> return Err("Near Gas: incorrect number entered".to_string()),
        }
    }
}

impl interactive_clap::ToCli for NearGas {
    type CliVariant = NearGas;
}
