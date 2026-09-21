use deribit::models::{Either, GetPositionsResponse, JSONRPCResponse};

#[test]
fn futures_positions_accept_integer_and_float_leverage() {
    for (leverage_json, expected) in [("25", 25.0), ("25.0", 25.0), ("35.639422", 35.639422)] {
        // Keep the wire representation: converting through an integer would hide this regression.
        let response = r#"{
            "jsonrpc": "2.0",
            "id": 2,
            "result": [{
                "size": 4.5e4,
                "kind": "future",
                "maintenance_margin": 0.4,
                "initial_margin": 0.6,
                "open_orders_margin": 0.0,
                "direction": "buy",
                "index_price": 2700.0,
                "instrument_name": "ETH-25DEC26",
                "settlement_price": 2690.0,
                "mark_price": 2760.0,
                "leverage": LEVERAGE,
                "delta": 16.3,
                "average_price": 1630.0,
                "floating_profit_loss": 0.4,
                "realized_profit_loss": 0.0,
                "total_profit_loss": 11.0,
                "size_currency": 16.3,
                "estimated_liquidation_price": null
            }],
            "usIn": 1789994941975814,
            "usOut": 1789994941976898,
            "usDiff": 1084,
            "testnet": false
        }"#
        .replace("LEVERAGE", leverage_json);

        let response: JSONRPCResponse<Vec<GetPositionsResponse>> =
            serde_json::from_str(&response).expect("future positions should deserialize");
        let positions = match response.result {
            Either::Left(positions) => positions,
            Either::Right(error) => panic!("unexpected RPC error: {:?}", error),
        };
        assert_eq!(positions.len(), 1);
        match &positions[0] {
            GetPositionsResponse::Future {
                leverage,
                size,
                estimated_liquidation_price,
                interest_value,
                realized_funding,
                ..
            } => {
                assert_eq!(*leverage, expected);
                assert_eq!(*size, 45_000.0);
                assert_eq!(*estimated_liquidation_price, None);
                assert_eq!(*interest_value, None);
                assert_eq!(*realized_funding, None);
            }
            _ => panic!("expected a future position"),
        }
    }
}
