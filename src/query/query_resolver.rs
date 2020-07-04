use crate::memory::memory_channel::MemoryChannel;
use crate::memory::memory_table;
use crate::memory::memory_table::MemoryTable;
use qdb_ast::ast::types::{DataType, FuncType, UnaryFuncExpr};
use qdb_ast::parser::states::DefaultParser;
use crate::memory::print_of_state::PrintOfState;
use std::borrow::BorrowMut;

pub struct QueryResolver;

pub enum QueryResponse {
    PrintOfStates(Vec<PrintOfState>),
    None
}

impl QueryResolver {
    pub fn resolve(mem_channel: &mut MemoryChannel, line: String) -> QueryResponse {
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
                    return QueryResponse::None;
                },

                FuncType::OnRead => {
                    let mut result : Vec<PrintOfState> = Vec::new();
                    for channel_data_type_name in unary_func_expr.get_channel_names() {
                        let maybe_channel_name = channel_data_type_name.symbol_to_string();
                        if maybe_channel_name.is_ok() {
                            if let Ok(channel_name) = maybe_channel_name {
                                let mem_table = mem_channel.get(&channel_name.to_string());
                                if mem_table.is_some() {
                                    for channel_binary_expressions in unary_func_expr.get_binary_exprs() {
                                        for binary_expression in channel_binary_expressions {
                                            let maybe_semi_result = mem_table.unwrap().find_by_predicate_intense(binary_expression);
                                            if maybe_semi_result.is_some() {
                                                result.append(maybe_semi_result.unwrap().borrow_mut());

                                            }
                                        }
                                    }

                                }

                            }

                        }

                    }
                    return QueryResponse::PrintOfStates(result)
                }
                _ => {}
            }
        }
       return  QueryResponse::None;
    }
}

mod test {
    use crate::memory::memory_channel::MemoryChannel;
    use crate::query::query_resolver::{QueryResolver,QueryResponse};

    #[test]
    fn test_query_resolver_resolve() {
        let mut a = MemoryChannel::new();
       QueryResolver::resolve(
            &mut a,
            "onCreate(my_node)(c:int = 2)".to_string(),
        );
       let aa =  QueryResolver::resolve(
            &mut a,
            "onRead(my_node)(c > 0)".to_string(),
        );

         if let QueryResponse::PrintOfStates(result) = aa {
             println!("{:?}",result)
         }

        //println!("{:#?}", a);
    }
}
