#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DashboardStatus {
    Loading,
    Ready,
    Empty,
    Degraded,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MetricDatum {
    pub title: &'static str,
    pub value: String,
    pub delta: String,
    pub positive: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ServiceDatum {
    pub service: &'static str,
    pub region: &'static str,
    pub status: &'static str,
    pub owner: &'static str,
    pub latency_ms: u32,
    pub alerts: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DashboardData {
    pub revision: u32,
    pub metrics: Vec<MetricDatum>,
    pub traffic: Vec<f64>,
    pub errors: Vec<f64>,
    pub api_capacity: Vec<f64>,
    pub worker_capacity: Vec<f64>,
    pub services: Vec<ServiceDatum>,
    pub cpu: f64,
    pub memory: f64,
    pub release_readiness: f64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DashboardFilters {
    pub query: String,
    pub region: RegionFilter,
    pub alerts_only: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RegionFilter {
    Global,
    Apac,
    Europe,
    Americas,
}

impl RegionFilter {
    pub const LABELS: [&'static str; 4] = ["Global", "APAC", "Europe", "Americas"];

    pub fn from_index(index: usize) -> Self {
        match index {
            1 => Self::Apac,
            2 => Self::Europe,
            3 => Self::Americas,
            _ => Self::Global,
        }
    }

    pub fn matches(self, region: &str) -> bool {
        matches!(self, Self::Global) || region.eq_ignore_ascii_case(self.label())
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Global => "Global",
            Self::Apac => "APAC",
            Self::Europe => "Europe",
            Self::Americas => "Americas",
        }
    }

    pub fn bias(self) -> f64 {
        match self {
            Self::Global => 0.0,
            Self::Apac => 18.0,
            Self::Europe => 8.0,
            Self::Americas => 12.0,
        }
    }
}

impl Default for DashboardFilters {
    fn default() -> Self {
        Self {
            query: String::new(),
            region: RegionFilter::Global,
            alerts_only: true,
        }
    }
}

impl DashboardData {
    pub fn generate(revision: u32, region: RegionFilter) -> Self {
        let wave = (revision % 7) as f64;
        let bias = region.bias();
        let scale = 1.0 + (revision % 5) as f64 * 0.025;
        let request_count = 1.18 + revision as f64 * 0.03 + bias / 100.0;
        let latency = (172.0 + bias + wave * 5.0).round();
        let error_rate = 0.12 + (revision % 4) as f64 * 0.025 + bias / 900.0;
        let slo = (99.96 - error_rate * 0.2).max(99.40);

        Self {
            revision,
            metrics: vec![
                MetricDatum {
                    title: "Requests",
                    value: format!("{request_count:.2}M"),
                    delta: format!("+{:.1}%", 9.4 + wave),
                    positive: true,
                },
                MetricDatum {
                    title: "Latency p95",
                    value: format!("{latency:.0}ms"),
                    delta: format!("-{:.1}%", 4.0 + wave / 2.0),
                    positive: true,
                },
                MetricDatum {
                    title: "Errors",
                    value: format!("{error_rate:.2}%"),
                    delta: format!("+{:.2}%", error_rate / 8.0),
                    positive: error_rate < 0.20,
                },
                MetricDatum {
                    title: "SLO",
                    value: format!("{slo:.2}%"),
                    delta: if slo > 99.85 { "healthy" } else { "watch" }.into(),
                    positive: slo > 99.85,
                },
            ],
            traffic: shifted_series(
                [120.0, 180.0, 160.0, 240.0, 310.0, 280.0, 360.0],
                bias,
                scale,
            ),
            errors: shifted_series([12.0, 18.0, 16.0, 20.0, 19.0, 24.0, 21.0], wave, 1.0),
            api_capacity: shifted_series(
                [72.0, 68.0, 81.0, 77.0, 85.0, 92.0, 88.0],
                bias / 3.0,
                1.0,
            ),
            worker_capacity: shifted_series(
                [42.0, 58.0, 61.0, 64.0, 69.0, 73.0, 78.0],
                bias / 4.0,
                1.0,
            ),
            services: services_for_revision(revision),
            cpu: (68.0 + wave * 3.0 + bias / 5.0).min(96.0),
            memory: (54.0 + wave * 2.0 + bias / 6.0).min(92.0),
            release_readiness: (88.0 + (revision % 6) as f64).min(99.0),
        }
    }
}

pub fn apply_filters<'a>(
    services: &'a [ServiceDatum],
    filters: &DashboardFilters,
) -> Vec<&'a ServiceDatum> {
    let query = filters.query.trim().to_lowercase();
    services
        .iter()
        .filter(|service| filters.region.matches(service.region))
        .filter(|service| !filters.alerts_only || service.alerts > 0)
        .filter(|service| {
            query.is_empty()
                || service.service.to_lowercase().contains(&query)
                || service.owner.to_lowercase().contains(&query)
                || service.status.to_lowercase().contains(&query)
        })
        .collect()
}

pub fn status_for(data: &DashboardData, visible_services: usize) -> DashboardStatus {
    if data.revision == 0 {
        DashboardStatus::Loading
    } else if visible_services == 0 {
        DashboardStatus::Empty
    } else if data
        .services
        .iter()
        .any(|service| service.status == "Degraded")
    {
        DashboardStatus::Degraded
    } else {
        DashboardStatus::Ready
    }
}

fn shifted_series(values: [f64; 7], shift: f64, scale: f64) -> Vec<f64> {
    values
        .into_iter()
        .enumerate()
        .map(|(index, value)| (value * scale + shift + index as f64 * 1.5).round())
        .collect()
}

fn services_for_revision(revision: u32) -> Vec<ServiceDatum> {
    let latency_shift = revision * 7;
    vec![
        ServiceDatum {
            service: "Gateway",
            region: "Global",
            status: "Healthy",
            owner: "Platform",
            latency_ms: 121 + latency_shift,
            alerts: 0,
        },
        ServiceDatum {
            service: "Billing",
            region: "Europe",
            status: "Watching",
            owner: "Revenue",
            latency_ms: 232 + latency_shift,
            alerts: 2,
        },
        ServiceDatum {
            service: "Search",
            region: "Americas",
            status: "Healthy",
            owner: "Discovery",
            latency_ms: 148 + latency_shift,
            alerts: 0,
        },
        ServiceDatum {
            service: "Jobs",
            region: "APAC",
            status: if revision % 3 == 0 {
                "Watching"
            } else {
                "Degraded"
            },
            owner: "Infra",
            latency_ms: 390 + latency_shift,
            alerts: 4,
        },
        ServiceDatum {
            service: "Analytics",
            region: "APAC",
            status: "Healthy",
            owner: "Data",
            latency_ms: 164 + latency_shift,
            alerts: u32::from(revision % 2 == 0),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_match_query_region_and_alerts() {
        let data = DashboardData::generate(1, RegionFilter::Global);
        let filters = DashboardFilters {
            query: "jobs".into(),
            region: RegionFilter::Apac,
            alerts_only: true,
        };
        let visible = apply_filters(&data.services, &filters);
        assert_eq!(visible.len(), 1);
        assert_eq!(visible[0].service, "Jobs");
    }

    #[test]
    fn empty_status_is_reported_for_no_visible_services() {
        let data = DashboardData::generate(1, RegionFilter::Global);
        let filters = DashboardFilters {
            query: "no-such-service".into(),
            region: RegionFilter::Global,
            alerts_only: false,
        };
        let visible = apply_filters(&data.services, &filters);
        assert_eq!(status_for(&data, visible.len()), DashboardStatus::Empty);
    }

    #[test]
    fn refresh_generation_changes_metrics_but_keeps_shape() {
        let first = DashboardData::generate(1, RegionFilter::Europe);
        let second = DashboardData::generate(2, RegionFilter::Europe);
        assert_eq!(first.metrics.len(), 4);
        assert_eq!(first.traffic.len(), second.traffic.len());
        assert_ne!(first.metrics[0].value, second.metrics[0].value);
    }
}
