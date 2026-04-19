// tests/benchmark_suite.rs

//! Efficiency and Power Usage benchmarks for the Penta-V Kernel.
//! Compares stability degradation across different geometric levels.

use penta_v_kernel::core::{KernelState, CORE_BASE};
use penta_v_kernel::core::cooling::CoolingProtocol; // استيراد البروتوكول الجديد
use penta_v_kernel::shapes::triangle::Triangle;
use penta_v_kernel::shapes::decagon::Decagon;
use penta_v_kernel::utils::calculator::calculate_and_apply_decay;

#[test]
fn test_efficiency_comparison() {
    let mut triangle_state = KernelState::default();
    let mut decagon_state = KernelState::default();
    
    // إنشاء كائن التبريد لإدارة الصدمات الحرارية
    let mut cooling = CoolingProtocol::new();

    let triangle = Triangle;
    let decagon = Decagon;

    // تم تحديث الاستدعاءات بإضافة &mut cooling كفارامتر رابع
    calculate_and_apply_decay(&mut triangle_state, 10.0, &triangle, &mut cooling);
    calculate_and_apply_decay(&mut decagon_state, 10.0, &decagon, &mut cooling);

    let triangle_loss = CORE_BASE - triangle_state.current_stability;
    let decagon_loss = CORE_BASE - decagon_state.current_stability;

    println!("Efficiency Report:");
    println!("Triangle loss: {:.4}", triangle_loss);
    println!("Decagon loss:  {:.4}", decagon_loss);

    // Verify Decagon is more efficient at dissipating the shock
    // وبما أن الديكاجون له أقطاب (Poles) أكثر، فإنه يمتص الصدمة بكفاءة أعلى
    assert!(decagon_loss < triangle_loss);
}
