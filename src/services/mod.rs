pub mod viacep;
pub mod cepla;
pub mod correios;

extern crate serde;
use serde::{Serialize, Deserialize};

// Should this struct be of Strings or strs ?
#[derive(Deserialize, Serialize, Debug)]
pub struct Address {
    pub cep: String,
    pub address: String,
    pub details: String,
    pub neighborhood: String,
    pub state: String,
    pub city: String,
}

impl viacep::Address {
    pub fn to_address(&self) -> Address{
        let addr = Address {
            cep: self.cep.clone(),
            address: self.address.clone(),
            details: self.details.clone(),
            neighborhood: self.neighborhood.clone(),
            state: self.state.clone(),
            city: self.city.clone(),  
        };
        addr
    }
}

impl correios::Address {
    pub fn to_address(&self) -> Address{
        let addr = Address {
            cep: self.cep.clone(),
            address: self.address.clone(),
            details: "".to_string(),
            neighborhood: self.neighborhood.clone(),
            state: self.state.clone(),
            city: self.city.clone(),  
        };
        addr
    }
}

impl cepla::Address {
    pub fn to_address(&self) -> Address{
        let addr = Address {
            cep: self.cep.clone(),
            address: self.address.clone(),
            details: self.details.clone(),
            neighborhood: self.neighborhood.clone(),
            state: self.state.clone(),
            city: self.city.clone(),  
        };
        addr
    }
}

#[cfg(test)]
mod tests {
    use super::viacep;
    use super::correios;
    use super::cepla;

    #[tokio::test]
    async fn viacep_conversion() {

        let viac_addr = viacep::Address {
            cep: "70150-903".to_string(),
            address: "SPP".to_string(),
            details: "Palácio da Alvorada (Residência Oficial do Presidente da República)".to_string(),
            neighborhood: "Zona Cívico-Administrativa".to_string(),
            city: "Brasília".to_string(),
            state: "DF".to_string(),
            unidade: "".to_string(),
            ibge: "5300108".to_string(),
            gia: "".to_string(),
        };
        let viac_addr = viac_addr.to_address();
        
        let addr = super::Address {
            cep: "70150-903".to_string(),
            state: "DF".to_string(),
            city: "Brasília".to_string(),
            neighborhood: "Zona Cívico-Administrativa".to_string(),
            address: "SPP".to_string(),
            details: "Palácio da Alvorada (Residência Oficial do Presidente da República)".to_string(),
        };

        assert_eq!(addr.address, viac_addr.address);
        assert_eq!(addr.state, viac_addr.state);
        assert_eq!(addr.neighborhood, viac_addr.neighborhood);
        assert_eq!(addr.city, viac_addr.city);
        assert_eq!(addr.cep, viac_addr.cep);
        assert_eq!(addr.details, viac_addr.details);
    }

    #[tokio::test]
    async fn correios_conversion() {

        let corr_addr = correios::Address {
            cep: "70150903".to_string(),
            state: "DF".to_string(),
            city: "Brasília".to_string(),
            neighborhood: "Zona Cívico-Administrativa".to_string(),
            address: "SPP".to_string(),
        };
        let corr_addr = corr_addr.to_address();

        let addr = super::Address {
            cep: "70150903".to_string(),
            state: "DF".to_string(),
            city: "Brasília".to_string(),
            neighborhood: "Zona Cívico-Administrativa".to_string(),
            address: "SPP".to_string(),
            details: "Palácio da Alvorada (Residência Oficial do Presidente da República)".to_string(),
        };

        assert_eq!(addr.address, corr_addr.address);
        assert_eq!(addr.state, corr_addr.state);
        assert_eq!(addr.neighborhood, corr_addr.neighborhood);
        assert_eq!(addr.city, corr_addr.city);
        assert_eq!(addr.cep, corr_addr.cep);
    }

    #[tokio::test]
    async fn cepla_conversion() {

        let cepl_addr = cepla::Address {
            cep: "70150903".to_string(),
            state: "DF".to_string(),
            city: "Brasília".to_string(),
            neighborhood: "Zona Cívico-Administrativa".to_string(),
            address: "SPP".to_string(),
            details: "Palácio da Alvorada (Residência Oficial do Presidente da República)".to_string(),
        };
        let cepl_addr = cepl_addr.to_address();
        
        let addr = super::Address {
            cep: "70150903".to_string(),
            state: "DF".to_string(),
            city: "Brasília".to_string(),
            neighborhood: "Zona Cívico-Administrativa".to_string(),
            address: "SPP".to_string(),
            details: "Palácio da Alvorada (Residência Oficial do Presidente da República)".to_string(),
        };

        assert_eq!(addr.address, cepl_addr.address);
        assert_eq!(addr.state, cepl_addr.state);
        assert_eq!(addr.neighborhood, cepl_addr.neighborhood);
        assert_eq!(addr.city, cepl_addr.city);
        assert_eq!(addr.cep, cepl_addr.cep);
        assert_eq!(addr.details, cepl_addr.details);
    }
}