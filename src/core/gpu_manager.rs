use std::sync::Arc;
use tokio::sync::mpsc;

pub struct GpuManager {
    device: Arc<Device>,
    queue: Queue,
    ml_model: MLModel,
}

impl GpuManager {
    pub fn new() -> Self {
        let instance = Instance::new(None).unwrap();
        let physical = PhysicalDevice::enumerate(&instance).next().unwrap();
        let device = Device::new(&physical, &Features::empty()).unwrap();
        let queue = device.get_queue(0, QueueFamily::COMPUTE);

        Self {
            device: Arc::new(device),
            queue,
            ml_model: MLModel::load(),
        }
    }

    pub async fn process_batch(&self, batch: Vec<PredictionInput>) -> Vec<PredictionResult> {
        // Обработка пакета данных на GPU
        todo!()
    }
}

z