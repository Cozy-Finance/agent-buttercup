/*
se criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use simulations::cozy::{
    configs::build_cozy_sim_runner_from_dir, runner::CozySingleSetSummaryGenerators,
};

fn from_num_agents(c: &mut Criterion) {
    let mut group = c.benchmark_group("from_num_agents");
    group.sample_size(10);

    for num_agents in [1, 10, 100].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(num_agents),
            num_agents,
            |b, &num_agents| {
                b.iter(|| {
                    let mut runner = build_cozy_sim_runner_from_dir("test").unwrap();
                    let workspace_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                        .to_str()
                        .unwrap();
                    let output_file_name = format!(
                        "{}/output/summaries/benches/{}_summary.json",
                        workspace_path, num_agents
                    );

                    runner.active_buyers_params.num_active = num_agents;
                    runner.passive_buyers_params.num_passive = num_agents;
                    runner.suppliers_params.num_passive = num_agents;

                    runner.run(
                        output_file_name.into(),
                        vec![
                            CozySingleSetSummaryGenerators::Set,
                            CozySingleSetSummaryGenerators::CostModels,
                        ],
                    );
                })
            },
        );
    }

    group.finish();
}

criterion_group!(benches, from_num_agents);
criterion_main!(benches);


*/

pub fn main() {
    println!("Hello, world!");
}
