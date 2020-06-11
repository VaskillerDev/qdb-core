use crate::memory::memory_channel::MemoryChannel;
use crate::memory::memory_table;
use crate::memory::memory_table::MemoryTable;
use qdb_ast::ast::types::{DataType, FuncType, UnaryFuncExpr};
use qdb_ast::parser::states::DefaultParser;

pub struct QueryResolver;

impl QueryResolver {
    pub fn resolve(mem_channel: &mut MemoryChannel, line: String) {
        let ast = DefaultParser::parse_from_string(line);
        for unary_func_expr in ast {
            let func_type = unary_func_expr.get_func_type();
            match func_type {
                FuncType::OnCreate => {
                    let vars = unary_func_expr.get_vars().as_ref().unwrap();

                    for channel_data_type_name in unary_func_expr.get_channel_names() {
                        let maybe_channel_name = channel_data_type_name.symbol_to_string();
                        if maybe_channel_name.is_ok() {
                            if let Ok(channel_name) = maybe_channel_name {
                                mem_channel.insert(channel_name.to_string(), MemoryTable::init());
                                let mem_table = mem_channel.get_mut(channel_name).unwrap();
                                vars.iter().for_each(|var| {
                                    let (name, value) = var.get();
                                    mem_table.insert(name, value.clone());
                                });
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

mod test {
    use crate::memory::memory_channel::MemoryChannel;
    use crate::query::query_resolver::QueryResolver;

    #[test]
    fn test_query_resolver_resolve() {
        let mut a = MemoryChannel::new();
        QueryResolver::resolve(
            &mut a,
            "onCreate(my_node)(a:int = 0,b : text = 2)".to_string(),
        );
        println!("{:#?}", a);
    }
}
