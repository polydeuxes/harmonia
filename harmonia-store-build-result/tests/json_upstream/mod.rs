//! Tests that verify JSON serialization matches upstream Nix format

use harmonia_store_build_result::{
    BuildResult, BuildResultFailure, BuildResultInner, BuildResultSuccess, FailureStatus,
    Microseconds, SuccessStatus,
};
use harmonia_store_derivation::derived_path::OutputName;
use harmonia_store_derivation::realisation::UnkeyedRealisation;
use harmonia_utils_test::json_upstream::libstore_test_data_path;
use harmonia_utils_test::test_upstream_json;

#[test]
fn resource_usage_json_success_and_failure() {
    for failure in [false, true] {
        let mut value = serde_json::json!({
            "success": !failure,
            "status": if failure { "PermanentFailure" } else { "Built" },
            "timesBuilt": 1,
            "startTime": 30,
            "stopTime": 50
        });
        if !failure {
            value["builtOutputs"] = serde_json::json!({});
        }
        let absent: BuildResult = serde_json::from_value(value.clone()).unwrap();
        assert!(absent.resource_usage.is_empty());
        assert!(
            serde_json::to_value(&absent)
                .unwrap()
                .get("resourceUsage")
                .is_none()
        );

        let statistics = serde_json::json!({
            "memoryPeak": 1073741824u64,
            "memorySwapPeak": 0,
            "unknownFutureStatistic": u64::MAX
        });
        value["resourceUsage"] = statistics.clone();
        let measured: BuildResult = serde_json::from_value(value.clone()).unwrap();
        let encoded = serde_json::to_value(&measured).unwrap();
        assert_eq!(encoded["resourceUsage"], statistics);
        assert_eq!(
            serde_json::from_value::<BuildResult>(encoded).unwrap(),
            measured
        );

        value["resourceUsage"] = serde_json::json!({});
        let empty: BuildResult = serde_json::from_value(value.clone()).unwrap();
        assert!(
            serde_json::to_value(empty)
                .unwrap()
                .get("resourceUsage")
                .is_none()
        );
        value["resourceUsage"] = serde_json::json!({"memoryPeak": -1});
        assert!(serde_json::from_value::<BuildResult>(value).is_err());
    }
}

test_upstream_json!(
    test_build_result_success,
    libstore_test_data_path("build-result/success.json"),
    {
        BuildResult {
            inner: BuildResultInner::Success(BuildResultSuccess {
                status: SuccessStatus::Built,
                built_outputs: [
                    (
                        "bar".parse::<OutputName>().unwrap(),
                        UnkeyedRealisation {
                            out_path: "g1w7hy3qg1w7hy3qg1w7hy3qg1w7hy3q-bar".parse().unwrap(),
                            signatures: Default::default(),
                        },
                    ),
                    (
                        "foo".parse::<OutputName>().unwrap(),
                        UnkeyedRealisation {
                            out_path: "g1w7hy3qg1w7hy3qg1w7hy3qg1w7hy3q-foo".parse().unwrap(),
                            signatures: Default::default(),
                        },
                    ),
                ]
                .into_iter()
                .collect(),
            }),
            times_built: 3,
            start_time: 30,
            stop_time: 50,
            cpu_user: Some(Microseconds(500000000)),
            cpu_system: Some(Microseconds(604000000)),
            resource_usage: Default::default(),
        }
    }
);

test_upstream_json!(
    test_build_result_output_rejected,
    libstore_test_data_path("build-result/output-rejected.json"),
    {
        BuildResult {
            inner: BuildResultInner::Failure(BuildResultFailure {
                status: FailureStatus::OutputRejected,
                error_msg: b"no idea why".to_vec(),
                is_non_deterministic: false,
            }),
            times_built: 3,
            start_time: 30,
            stop_time: 50,
            cpu_user: None,
            cpu_system: None,
            resource_usage: Default::default(),
        }
    }
);

test_upstream_json!(
    test_build_result_not_deterministic,
    libstore_test_data_path("build-result/not-deterministic.json"),
    {
        BuildResult {
            inner: BuildResultInner::Failure(BuildResultFailure {
                status: FailureStatus::NotDeterministic,
                error_msg: b"no idea why".to_vec(),
                is_non_deterministic: false,
            }),
            times_built: 1,
            start_time: 0,
            stop_time: 0,
            cpu_user: None,
            cpu_system: None,
            resource_usage: Default::default(),
        }
    }
);
