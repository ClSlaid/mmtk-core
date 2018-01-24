use super::super::Scheduling;
use ::plan::ParallelCollector;

pub struct VMScheduling {}

impl Scheduling for VMScheduling {
    fn stop_all_mutators(thread_id: usize) {
        unimplemented!();
    }

    fn resume_mutators(thread_id: usize) {
        unimplemented!();
    }

    fn block_for_gc(thread_id: usize) {
        unimplemented!();
    }

    unsafe fn spawn_worker_thread<T: ParallelCollector>(thread_id: usize, ctx: *mut T) {
        unimplemented!();
    }
}