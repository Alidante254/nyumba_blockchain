use crate::enums::user_enums::*;
use candid::CandidType;
use serde::{Deserialize, Serialize};

// Struct to hold user data common to all users
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub name: String,
    pub email: String,
    pub password: String,
    pub birth_date: String,
    pub photo_url: String,
    pub gender: Gender,
}

// Struct to represent normal users
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct NormalUser {
    pub user_data: User,
    pub normal_user_role: NormalUserRole,
    pub is_verified: bool,
    pub verification_documents: Vec<u64>,
}

// Struct to represent buyers
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Buyer {
    pub normal_user: NormalUser,
    pub cart: Option<u64>,
    pub orders: Option<Vec<u64>>,
}

// Struct to represent sellers
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Seller {
    pub normal_user: NormalUser,
    pub properties: Option<Vec<u64>>,
    pub orders: Option<Vec<u64>>,
}

// Struct to represent super users
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct SuperUser {
    pub user_data: User,
    pub super_user_role: SuperUserRole,
}

// Struct to represent admin users
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Admin {
    pub super_user_role: SuperUser,
    pub work_role: AdminWorkRole,
}

// Struct to represent officials
#[derive(Debug)]
pub struct Official {
    pub super_user: SuperUser,
    pub work_role: OfficialWorkRole,
}
