use serde::{Deserialize, Serialize};
use std::time::Duration;

pub const METRIC_SAMPLE_INTERVAL: Duration = Duration::from_secs(1);

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MetricBoolFlag {
    pub available: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProcessMetrics {
    pub available: bool,
    pub rss_kib: u64,
    pub peak_rss_kib: u64,
    pub threads: usize,
    pub swap_kib: u64,
    pub anonymous_kib: u64,
    pub file_kib: u64,
    pub shmem_kib: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SmapsRollupMetrics {
    pub available: bool,
    pub rss_kib: u64,
    pub pss_kib: u64,
    pub private_kib: u64,
    pub shared_kib: u64,
    pub swap_kib: u64,
    pub swap_pss_kib: u64,
    pub pss_anon_kib: u64,
    pub pss_file_kib: u64,
    pub pss_shmem_kib: u64,
    pub anonymous_kib: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MappingCategory {
    pub category: String,
    pub rss_kib: u64,
    pub pss_kib: u64,
    pub private_kib: u64,
    pub shared_kib: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MetricCategories {
    pub available: bool,
    pub categories: Vec<MappingCategory>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CgroupMetrics {
    pub available: bool,
    pub version: String,
    pub current_bytes: u64,
    pub peak_bytes: u64,
    pub max_bytes: Option<u64>,
    pub swap_current_bytes: Option<u64>,
    pub swap_max_bytes: Option<u64>,
    pub anon_bytes: u64,
    pub file_bytes: u64,
    pub shmem_bytes: u64,
    pub sock_bytes: u64,
    pub kernel_stack_bytes: u64,
    pub pagetables_bytes: u64,
    pub percpu_bytes: u64,
    pub slab_reclaimable_bytes: u64,
    pub slab_unreclaimable_bytes: u64,
    pub file_mapped_bytes: u64,
    pub file_dirty_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SocketMetrics {
    pub available: bool,
    pub socket_fds: usize,
    pub udp_sockets: usize,
    pub udp_tx_queue_bytes: u64,
    pub udp_rx_queue_bytes: u64,
    pub tcp_sockets: usize,
    pub tcp_tx_queue_bytes: u64,
    pub tcp_rx_queue_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BufferMetrics {
    pub available: bool,
    pub packet_pool_capacity_slots: usize,
    pub packet_pool_allocated_slots: usize,
    pub packet_pool_allocated_payload_bytes: u64,
    pub packet_pool_retained_slots: usize,
    pub udp_tx_slots: usize,
    pub packet_capacity_bytes: u64,
    pub udp_tx_payload_bytes: u64,
    pub udp_tx_in_use_slots: usize,
    pub tun_tx_slots: usize,
    pub tun_tx_payload_bytes: u64,
    pub tun_tx_in_use_slots: usize,
    pub udp_rx_mode: String,
    pub udp_rx_slots: usize,
    pub udp_rx_slot_bytes: u64,
    pub udp_rx_payload_bytes: u64,
    pub tun_rx_payload_bytes: u64,
    pub fixed_payload_bytes: u64,
    pub udp_socket_rcvbuf_request_bytes: u64,
    pub udp_socket_sndbuf_request_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RuntimeMetrics {
    pub available: bool,
    pub allocator: String,
    pub hot_sessions: usize,
    pub hot_session_capacity: usize,
    pub hot_session_limit: usize,
    pub public_sessions: usize,
    pub public_session_capacity: usize,
    pub max_stream_workers_per_device: usize,
    pub engine_epochs: usize,
    pub device_epochs: usize,
    pub device_epoch_capacity: usize,
    pub derived_keys: usize,
    pub derived_key_string_capacity_bytes: u64,
    pub web_sessions: usize,
    pub web_session_limit: usize,
    pub web_session_key_capacity_bytes: u64,
    pub login_limits: usize,
    pub login_limit_limit: usize,
    pub login_limit_key_capacity_bytes: u64,
    pub log_entries: usize,
    pub log_entry_limit: usize,
    pub log_string_capacity_bytes: u64,
    pub log_ring_metadata_capacity_bytes: u64,
    pub dpi_entries: usize,
    pub dpi_entry_capacity: usize,
    pub dpi_retained_bytes: u64,
    pub stream_repairs: usize,
    pub stream_inventory: usize,
    pub dataplane_commands_queued: usize,
    pub dataplane_command_capacity: usize,
    pub memory_trim_count: usize,
    pub local_proxy_active: bool,
    pub local_proxy_tcp_sessions: usize,
    pub local_proxy_tcp_limit: usize,
    pub local_proxy_udp_flows: usize,
    pub local_proxy_udp_limit: usize,
    pub local_proxy_payload_allocated_bytes: u64,
    pub local_proxy_payload_retained_bytes: u64,
    pub local_proxy_opening_buffer_allocated_bytes: u64,
    pub local_proxy_payload_upper_bound_at_limit_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StorageMetrics {
    pub available: bool,
    pub sqlite_db_bytes: u64,
    pub sqlite_wal_bytes: u64,
    pub sqlite_shm_bytes: u64,
    pub log_file_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MetricFrame {
    pub error: String,
    pub timestamp_ms: u64,
    pub pid: u32,
    pub allocated_bytes: u64,
    pub retained_bytes: u64,
    pub process: ProcessMetrics,
    pub smaps_rollup: SmapsRollupMetrics,
    pub mappings: MetricCategories,
    pub cgroup: CgroupMetrics,
    pub sockets: SocketMetrics,
    pub fixed_buffers: BufferMetrics,
    pub runtime: RuntimeMetrics,
    pub storage: StorageMetrics,
}

pub async fn collect_metric_frame<T>(_app: &T) -> MetricFrame {
    MetricFrame::default()
}

pub fn allocated_bytes() -> usize {
    0
}

pub fn retained_bytes() -> usize {
    0
}

pub fn available_memory_bytes() -> Option<u64> {
    None
}

pub fn collect_allocator_thread_heap() -> serde_json::Value {
    serde_json::Value::Null
}
