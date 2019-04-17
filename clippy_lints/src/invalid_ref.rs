use crate::utils::{paths, span_help_and_lint};
use if_chain::if_chain;
use rustc::hir::*;
use rustc::lint::{LateContext, LateLintPass, LintArray, LintPass};
use rustc::ty;
use rustc::{declare_tool_lint, lint_array};

declare_clippy_lint! {
    /// **What it does:** Checks for creation of references to zeroed or uninitialized memory.
    ///
    /// **Why is this bad?** Creation of null references is undefined behavior.
    ///
    /// **Known problems:** None.
    ///
    /// **Example:**
    /// ```no_run
    /// let bad_ref: &usize = unsafe { std::mem::zeroed() };
    /// ```
    pub INVALID_REF,
    correctness,
    "creation of invalid reference"
}

const ZERO_REF_SUMMARY: &str = "reference to zeroed memory";
const UNINIT_REF_SUMMARY: &str = "reference to uninitialized memory";
const HELP: &str = "Creation of a null reference is undefined behavior; \
                    see https://doc.rust-lang.org/reference/behavior-considered-undefined.html";

pub struct InvalidRef;

impl LintPass for InvalidRef {
    fn get_lints(&self) -> LintArray {
        lint_array!(INVALID_REF)
    }

    fn name(&self) -> &'static str {
        "InvalidRef"
    }
}

impl<'a, 'tcx> LateLintPass<'a, 'tcx> for InvalidRef {
    fn check_expr(&mut self, cx: &LateContext<'a, 'tcx>, expr: &'tcx Expr) {
        if_chain! {
            if let ExprKind::Call(ref path, ref args) = expr.node;
            if let ExprKind::Path(ref qpath) = path.node;
            if args.len() == 0;
            if let ty::Ref(..) = cx.tables.expr_ty(expr).sty;
            if let Some(def_id) = cx.tables.qpath_def(qpath, path.hir_id).opt_def_id();
            then {
                let msg = if cx.match_def_path(def_id, &paths::MEM_ZEROED) |
                             cx.match_def_path(def_id, &paths::INIT)
                {
                    ZERO_REF_SUMMARY
                } else if cx.match_def_path(def_id, &paths::MEM_UNINIT) |
                          cx.match_def_path(def_id, &paths::UNINIT)
                {
                    UNINIT_REF_SUMMARY
                } else {
                    return;
                };
                span_help_and_lint(cx, INVALID_REF, expr.span, msg, HELP);
            }
        }
        return;
    }
}
