use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct AuditInput {
    pub btc_reserve: f64,
    pub btc_price: f64,
    pub sdac_supply: f64,
}

#[derive(Serialize, Deserialize)]
pub struct AuditResult {
    pub reserve_ratio: f64,
    pub is_solvent: bool,
    pub compliance_score: f64,
    pub risk_level: String,
}

#[wasm_bindgen]
pub fn audit_compliance(json_input: &str) -> String {
    let input: AuditInput = serde_json::from_str(json_input).unwrap_or(AuditInput {
        btc_reserve: 0.0,
        btc_price: 0.0,
        sdac_supply: 1.0,
    });

    let reserve_value = input.btc_reserve * input.btc_price;
    let ratio = reserve_value / input.sdac_supply;
    
    let is_solvent = ratio >= 1.0;
    
    // 점수 계산 로직: 1.0x를 기준으로 100점 만점 설계
    let mut score = (ratio * 100.0).min(100.0);
    if !is_solvent {
        score = score * 0.8; // 지급 불능 시 패널티
    }

    let risk_level = if ratio > 1.05 {
        "SECURE".to_string()
    } else if ratio >= 1.0 {
        "STABLE".to_string()
    } else if ratio > 0.95 {
        "CAUTION".to_string()
    } else {
        "CRITICAL".to_string()
    };

    let result = AuditResult {
        reserve_ratio: ratio,
        is_solvent,
        compliance_score: score,
        risk_level,
    };

    serde_json::to_string(&result).unwrap()
}
