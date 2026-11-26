#[test]
pub fn be_test() {
    use crate::op;
    use crate::Module;
    use crate::reflect::{Reflector, ResourceType};

    assert_ne!(include_bytes!("../mesh.spv"), include_bytes!("../be.spv"));

    let spirv = crate::include_spirv!("../be.spv"); 
    let module = Module::new(spirv).unwrap();
    let mut reflector = Reflector::new(module).unwrap();
    reflector.set_entry_point("niam", op::ExecutionModel::MESH_EXT).unwrap();
    for ubo in reflector.resources_for_type(ResourceType::UniformBuffer).unwrap() {
        match ubo {
            Ok(ubo) => {
                let mut set = None;
                let mut binding = None;
                for dec in reflector.decorations(ubo.variable_id) {
                    if let op::DecorationOperands::DescriptorSet { descriptor_set } = dec.decoration {
                        set = Some(descriptor_set);
                    } else if let op::DecorationOperands::Binding { binding_point } = dec.decoration {
                        binding = Some(binding_point);
                    }
                }
                println!("Uniform buffer (set {}, binding {}): {}",
                    set.unwrap(), binding.unwrap(), ubo.name.unwrap_or_default(),
                );
            },
            Err(err) => eprintln!("parse error: {err}")
        }
    }
    for pc in reflector.resources_for_type(ResourceType::PushConstant).unwrap() {
        match pc {
            Ok(pc) => {
                let offset = pc.offset.unwrap();
                let size = reflector
                    .type_description(pc.base_type_id)
                    .unwrap().size_hint.declared();
                println!("Push constant (offset {offset}, size {size}): {}", pc.name.unwrap_or_default());
            },
            Err(err) => eprintln!("parse error: {err}"),
        }
    }
}

#[test]
fn spec_test() {
    use crate::Module;
    use crate::Literal;
    use crate::reflect::{Reflector, SpecConstantValue, ScalarType, LocalSize};
    use crate::include_spirv;
    fn test(spirv: &[u32], ep: &str) {
        let module = Module::new(spirv).unwrap();
        let mut reflector = Reflector::new(module).unwrap();
        let mut spec_constants = vec![];
        for spec in reflector.spec_constants() {
            spec_constants.push(spec.unwrap());
        }
        spec_constants.sort_by_key(|s| s.spec_id);
        let values: Vec<_> = spec_constants
            .iter()
            .map(|v| (v.spec_id, v.value, v.ty))
            .collect();
        assert_eq!(values,
            [
                (0, SpecConstantValue::True, ScalarType::Bool),
                (1, SpecConstantValue::False, ScalarType::Bool),
                (2, SpecConstantValue::Literal(Literal::F32(42.0)), ScalarType::Float { width: 32 }),
                (3, SpecConstantValue::Literal(Literal::U64(!42)),
                    ScalarType::Int { width: 64, is_signed: false, },
                ),
                (4, SpecConstantValue::Literal(Literal::U32(1)),
                    ScalarType::Int { width: 32, is_signed: false, }
                ),
                (4, SpecConstantValue::Literal(Literal::U32(1)),
                    ScalarType::Int { width: 32, is_signed: false, }
                ),
            ]
        );
        reflector.set_entry_point(
            ep,
            crate::op::ExecutionModel::GLCOMPUTE,
        ).unwrap();
        let local_size = reflector.local_size().unwrap();
        std::assert_matches!(local_size, Some(LocalSize::Id(_, _, _)));
    }
    assert_ne!(include_bytes!("../spec_test.spv"), include_bytes!("../spec_test_be.spv"));
    test(include_spirv!("../spec_test.spv"), "main");
    test(include_spirv!("../spec_test_be.spv"), "niam");
}
