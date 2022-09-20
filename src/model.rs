use std::str::FromStr;

use mongodb::{
    bson::{doc, oid::ObjectId, to_bson, Bson, DateTime},
    Collection, Database,
};
use serde::{de, Deserialize, Serialize, Serializer};
use strum::{Display, EnumString};

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum OrganizationPricingTier {
    Free = 0,
    T1 = 1,
    T2 = 2,
    T3 = 3,
    T4 = 4,
    T5 = 5,
}

impl From<OrganizationPricingTier> for Bson {
    fn from(value: OrganizationPricingTier) -> Self {
        to_bson(&value).unwrap()
    }
}

impl ToString for OrganizationPricingTier {
    fn to_string(&self) -> String {
        match self {
            Self::Free => String::from("FREE"),
            Self::T1 => String::from("T1"),
            Self::T2 => String::from("T2"),
            Self::T3 => String::from("T3"),
            Self::T4 => String::from("T4"),
            Self::T5 => String::from("T5"),
        }
    }
}

impl OrganizationPricingTier {
    pub fn info(&self) -> OrganizationPricing {
        match self {
            Self::Free => OrganizationPricing::free(),
            Self::T1 => OrganizationPricing::t1(),
            Self::T2 => OrganizationPricing::t2(),
            Self::T3 => OrganizationPricing::t3(),
            Self::T4 => OrganizationPricing::t4(),
            Self::T5 => OrganizationPricing::t5(),
        }
    }

    pub fn features(&self) -> Vec<OrganizationFeatures> {
        self.info().features
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumString)]
pub enum OrganizationFeatures {
    #[strum(serialize = "CashRegister")]
    CashRegister,
    #[strum(serialize = "AccountantAccess")]
    AccountantAccess,
    #[strum(serialize = "OnlineCustomerAndVendorPortal")]
    OnlineCustomerAndVendorPortal,
    #[strum(serialize = "OnlineGstAccess")]
    OnlineGstAccess,
    #[strum(serialize = "Website")]
    Website,
    #[strum(serialize = "SaleIncharge")]
    SaleIncharge,
    #[strum(serialize = "ChequePrinting")]
    ChequePrinting,
    #[strum(serialize = "MaterialConversion")]
    MaterialConversion,
    #[strum(serialize = "Warehouse")]
    Warehouse,
    #[strum(serialize = "MultipleGST")]
    MultipleGST,
    #[strum(serialize = "EmailIntegration")]
    EmailIntegration,
}

impl From<OrganizationFeatures> for Bson {
    fn from(value: OrganizationFeatures) -> Self {
        to_bson(&value).unwrap()
    }
}

impl Serialize for OrganizationFeatures {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for OrganizationFeatures {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        FromStr::from_str(&s).map_err(de::Error::custom)
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrganizationPricing {
    pub tier: OrganizationPricingTier,
    pub price: usize,
    pub features: Vec<OrganizationFeatures>,
    pub max_users: Option<usize>,
    pub max_branches: Option<usize>,
    pub branches: usize,
    pub users: usize,
    pub cash_registers: usize,
    pub vouchers: usize,
    pub storage: usize,
    pub clients: usize,
    pub warehouse: usize,
}

impl OrganizationPricing {
    pub fn free() -> OrganizationPricing {
        OrganizationPricing {
            tier: OrganizationPricingTier::Free,
            price: 0,
            features: vec![],
            max_users: Some(1),
            max_branches: Some(1),
            users: 1,
            branches: 1,
            cash_registers: 0,
            vouchers: 1200,
            storage: 500,
            clients: 0,
            warehouse: 0,
        }
    }

    pub fn t1() -> OrganizationPricing {
        OrganizationPricing {
            tier: OrganizationPricingTier::T1,
            price: 399,
            features: OrganizationPricingTier::Free
                .features()
                .into_iter()
                .chain(vec![])
                .collect(),
            max_users: None,
            max_branches: Some(1),
            users: 5,
            branches: 1,
            cash_registers: 0,
            vouchers: 1200,
            storage: 500,
            clients: 1,
            warehouse: 0,
        }
    }

    pub fn t2() -> OrganizationPricing {
        OrganizationPricing {
            tier: OrganizationPricingTier::T2,
            price: 999,
            features: OrganizationPricingTier::T1
                .features()
                .into_iter()
                .chain(vec![OrganizationFeatures::AccountantAccess])
                .collect(),
            max_users: None,
            max_branches: Some(2),
            users: 10,
            branches: 1,
            cash_registers: 0,
            vouchers: 6000,
            storage: 500,
            clients: 5,
            warehouse: 0,
        }
    }

    pub fn t3() -> OrganizationPricing {
        OrganizationPricing {
            tier: OrganizationPricingTier::T3,
            price: 2499,
            features: OrganizationPricingTier::T2
                .features()
                .into_iter()
                .chain(vec![
                    OrganizationFeatures::SaleIncharge,
                    OrganizationFeatures::EmailIntegration,
                    OrganizationFeatures::Website,
                    OrganizationFeatures::OnlineGstAccess,
                    OrganizationFeatures::CashRegister,
                ])
                .collect(),
            max_users: None,
            max_branches: Some(3),
            users: 20,
            branches: 1,
            cash_registers: 7,
            vouchers: 12000,
            storage: 1000,
            clients: 12,
            warehouse: 0,
        }
    }

    pub fn t4() -> OrganizationPricing {
        OrganizationPricing {
            tier: OrganizationPricingTier::T4,
            price: 6499,
            features: OrganizationPricingTier::T3
                .features()
                .into_iter()
                .chain(vec![
                    OrganizationFeatures::ChequePrinting,
                    OrganizationFeatures::MaterialConversion,
                    OrganizationFeatures::Warehouse,
                ])
                .collect(),
            max_users: None,
            max_branches: Some(4),
            users: 40,
            branches: 1,
            cash_registers: 18,
            vouchers: 30000,
            storage: 2000,
            clients: 30,
            warehouse: 1,
        }
    }

    pub fn t5() -> OrganizationPricing {
        OrganizationPricing {
            tier: OrganizationPricingTier::T5,
            price: 14999,
            features: OrganizationPricingTier::T4
                .features()
                .into_iter()
                .chain(vec![
                    OrganizationFeatures::OnlineCustomerAndVendorPortal,
                    OrganizationFeatures::MultipleGST,
                ])
                .collect(),
            max_users: None,
            max_branches: None,
            users: 100,
            branches: 1,
            cash_registers: 50,
            vouchers: 80000,
            storage: 4000,
            clients: 70,
            warehouse: 2,
        }
    }
}

#[derive(Eq, Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum OrganizationStatus {
    Active,
    Suspended,
    Deactivated,
}

impl From<OrganizationStatus> for Bson {
    fn from(value: OrganizationStatus) -> Self {
        to_bson(&value).unwrap()
    }
}

impl ToString for OrganizationStatus {
    fn to_string(&self) -> String {
        match self {
            Self::Active => String::from("ACTIVE"),
            Self::Suspended => String::from("SUSPENDED"),
            Self::Deactivated => String::from("DEACTIVATED"),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrganizationPricingAdditions {
    pub branches: usize,
    pub users: usize,
    pub cash_registers: usize,
    pub clients: usize,
    pub warehouse: usize,
}
impl From<OrganizationPricingAdditions> for Bson {
    fn from(additions: OrganizationPricingAdditions) -> Self {
        let additions_doc = doc! {
            "branches": additions.branches as u32,
            "users": additions.users as u32,
            "cashRegisters": additions.cash_registers as u32,
            "clients": additions.clients as u32,
            "warehouse": additions.warehouse as u32,
        };
        Bson::Document(additions_doc)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrganizationAddress {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pin_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    pub country: String,
}

impl OrganizationAddress {
    pub fn with_country(country: String) -> OrganizationAddress {
        OrganizationAddress {
            street: None,
            city: None,
            pin_code: None,
            state: None,
            country,
        }
    }
}

impl From<OrganizationAddress> for Bson {
    fn from(value: OrganizationAddress) -> Self {
        to_bson(&value).unwrap()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Organization {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    pub full_name: String,
    pub country: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gst_no: Option<String>,
    pub book_begin: DateTime,
    pub fp_code: u8,
    pub pricing: OrganizationPricingTier,
    pub cluster: String,
    pub users: Vec<ObjectId>,
    pub communication_address: OrganizationAddress,
    pub billing_address: OrganizationAddress,
    pub grace_period: u8,
    pub unbilled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additions: Option<OrganizationPricingAdditions>,
    pub status: OrganizationStatus,
    #[serde(default)]
    pub fund: usize,
    pub owned_by: ObjectId,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl Organization {
    pub fn collection(db: &Database) -> Collection<Self> {
        db.collection("organizations")
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrganizationUsage {
    pub billing_period: String,
    pub plan: String,
    pub base_charge: f32,
    pub additional_usage_charges: f32,
}

impl From<OrganizationUsage> for Bson {
    fn from(usage: OrganizationUsage) -> Self {
        to_bson(&usage).unwrap()
    }
}

#[derive(Eq, Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum PaidStatus {
    Paid,
    Unpaid,
}

impl From<PaidStatus> for Bson {
    fn from(value: PaidStatus) -> Self {
        to_bson(&value).unwrap()
    }
}

impl ToString for PaidStatus {
    fn to_string(&self) -> String {
        match self {
            Self::Paid => String::from("PAID"),
            Self::Unpaid => String::from("UNPAID"),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Invoice {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub invoice_no: u8,
    pub date: DateTime,
    pub billed_to: String,
    pub organization: String,
    pub organization_usage: Vec<OrganizationUsage>,
    pub service_value: f32,
    pub tax_ratio: f32,
    pub tax_value: f32,
    pub total_value: f32,
    pub rounded_value: f32,
    pub draft: bool,
    pub paid_status: PaidStatus,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl Invoice {
    pub fn collection(db: &Database) -> Collection<Self> {
        db.collection("invoices")
    }
}
