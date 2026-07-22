use serde::{Deserialize, Serialize};

use super::common::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plan {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<PlanAmount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_fee: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<PlanInterval>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial: Option<PlanTrial>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_cycles: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_methods: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanAmount {
    pub value: i64,
    #[serde(default = "default_currency")]
    pub currency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanInterval {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanTrial {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePlanRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub amount: PlanAmount,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_fee: Option<i64>,
    pub interval: PlanInterval,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial: Option<PlanTrial>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_cycles: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_methods: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editable: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePlanRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<PlanAmount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_fee: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<PlanInterval>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial: Option<PlanTrial>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_cycles: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_methods: Option<Vec<String>>,
}
