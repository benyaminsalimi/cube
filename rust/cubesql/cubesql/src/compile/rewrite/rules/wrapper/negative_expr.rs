use crate::{
    compile::rewrite::{
        negative_expr, rewrite,
        rewriter::{CubeEGraph, CubeRewrite},
        rules::wrapper::WrapperRules,
        transforming_rewrite, wrapper_pullup_replacer, wrapper_pushdown_replacer,
        wrapper_replacer_context, WrapperReplacerContextAliasToCube,
    },
    var, var_iter,
};
use egg::Subst;

impl WrapperRules {
    pub fn negative_expr_rules(&self, rules: &mut Vec<CubeRewrite>) {
        rules.extend(vec![
            rewrite(
                "wrapper-negative-push-down",
                wrapper_pushdown_replacer(negative_expr("?expr"), "?context"),
                negative_expr(wrapper_pushdown_replacer("?expr", "?context")),
            ),
            transforming_rewrite(
                "wrapper-negative-pull-up",
                negative_expr(wrapper_pullup_replacer(
                    "?expr",
                    wrapper_replacer_context(
                        "?alias_to_cube",
                        "?push_to_cube",
                        "?in_projection",
                        "?cube_members",
                        "?grouped_subqueries",
                        "?ungrouped_scan",
                    ),
                )),
                wrapper_pullup_replacer(
                    negative_expr("?expr"),
                    wrapper_replacer_context(
                        "?alias_to_cube",
                        "?push_to_cube",
                        "?in_projection",
                        "?cube_members",
                        "?grouped_subqueries",
                        "?ungrouped_scan",
                    ),
                ),
                self.transform_negative_expr("?alias_to_cube"),
            ),
        ]);
    }

    fn transform_negative_expr(
        &self,
        alias_to_cube_var: &'static str,
    ) -> impl Fn(&mut CubeEGraph, &mut Subst) -> bool {
        let alias_to_cube_var = var!(alias_to_cube_var);
        let meta = self.meta_context.clone();
        move |egraph, subst| {
            for alias_to_cube in var_iter!(
                egraph[subst[alias_to_cube_var]],
                WrapperReplacerContextAliasToCube
            )
            .cloned()
            {
                if let Some(sql_generator) = meta.sql_generator_by_alias_to_cube(&alias_to_cube) {
                    if sql_generator
                        .get_sql_templates()
                        .templates
                        .contains_key("expressions/negative")
                    {
                        return true;
                    }
                }
            }
            false
        }
    }
}
