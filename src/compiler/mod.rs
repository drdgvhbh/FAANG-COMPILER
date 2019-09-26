use inkwell::{
    builder::Builder,
    context::Context,
    module::Module,
    types::StructType,
    values::{BasicValueEnum, PointerValue},
    AddressSpace,
};

pub mod external;
pub mod stdlib;
mod util;

pub struct Compiler<'a> {
    context: &'a Context,
    builder: &'a Builder,
    module: &'a Module,
}

impl<'a> Compiler<'a> {
    pub fn new(context: &'a Context, module: &'a Module, builder: &'a Builder) -> Compiler<'a> {
        Compiler {
            module,
            context,
            builder,
        }
    }

    pub fn compile(&mut self, program: &super::ast::Expression) -> Result<&Module, String> {
        let main_fcn =
            self.module
                .add_function("main", self.context.i32_type().fn_type(&[], false), None);
        let block = self.context.append_basic_block(&main_fcn, "entry");
        self.builder.position_at_end(&block);

        match program {
            super::ast::Expression::Invocation(name, args) => {
                let fcn_name = format!("FAANG_{}", name);
                let fcn = self
                    .module
                    .get_function(&fcn_name)
                    .expect(&format!("function {} should exist", name));
                self.builder.build_call(
                    fcn,
                    &args
                        .iter()
                        .map(|arg| self.expression_to_basic_value_enum(arg).unwrap())
                        .collect::<Vec<_>>(),
                    "val",
                );
            }
            super::ast::Expression::StringLiteral(lit) => {
                return Err("not implemented".into());
            }
        }

        self.builder
            .build_return(Some(&self.context.i32_type().const_int(0, false)));

        Ok(&self.module)
    }

    fn expression_to_basic_value_enum(
        &self,
        expression: &super::ast::Expression,
    ) -> Result<BasicValueEnum, String> {
        match expression {
            super::ast::Expression::Invocation(name, args) => Err("not impelemented".into()),
            super::ast::Expression::StringLiteral(lit) => {
                Ok(util::alloc_string(&lit, "str", self.context, self.builder).into())
            }
        }
    }
}
