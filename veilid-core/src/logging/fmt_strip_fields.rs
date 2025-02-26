use super::*;
use tracing_subscriber::{
    field::{MakeVisitor, RecordFields, Visit, VisitOutput},
    fmt::{
        format::{DefaultFields, Writer},
        FormatFields,
    },
};

#[must_use]
pub struct FmtStripFields {
    /// The inner formatter that will be used to format fields
    fmt: DefaultFields,
    /// The names of fields to remove
    remove_keys: HashSet<&'static str>,
}

impl FmtStripFields {
    /// Create a new [`FmtStripFields`] formatter based on the given `fmt`,
    /// that removes fields with a given set of names
    pub fn new(fmt: DefaultFields, remove_keys: HashSet<&'static str>) -> Self {
        Self { fmt, remove_keys }
    }
}

impl<'writer> FormatFields<'writer> for FmtStripFields {
    fn format_fields<R: RecordFields>(&self, writer: Writer<'writer>, fields: R) -> fmt::Result {
        let mut visitor = FmtStripVisitor::new(self.fmt.make_visitor(writer), &self.remove_keys);
        fields.record(&mut visitor);
        visitor.finish()
    }
}

struct FmtStripVisitor<'a, F, Out>
where
    F: Visit + VisitOutput<Out>,
{
    remove_keys: &'a HashSet<&'static str>,
    visitor: F,
    _phantom: core::marker::PhantomData<Out>,
}

impl<'a, F, Out> FmtStripVisitor<'a, F, Out>
where
    F: Visit + VisitOutput<Out>,
{
    pub fn new(visitor: F, remove_keys: &'a HashSet<&'static str>) -> Self {
        Self {
            remove_keys,
            visitor,
            _phantom: core::marker::PhantomData {},
        }
    }
}

impl<'a, F, Out> VisitOutput<Out> for FmtStripVisitor<'a, F, Out>
where
    F: Visit + VisitOutput<Out>,
{
    fn finish(self) -> Out {
        self.visitor.finish()
    }

    fn visit<R>(self, fields: &R) -> Out
    where
        R: RecordFields,
        Self: Sized,
    {
        self.visitor.visit(fields)
    }
}

impl<'a, F, Out> Visit for FmtStripVisitor<'a, F, Out>
where
    F: Visit + VisitOutput<Out>,
{
    fn record_debug(&mut self, field: &field::Field, value: &dyn fmt::Debug) {
        if self.remove_keys.contains(field.name()) {
            return;
        }
        self.visitor.record_debug(field, value);
    }

    fn record_f64(&mut self, field: &field::Field, value: f64) {
        if self.remove_keys.contains(field.name()) {
            return;
        }
        self.visitor.record_f64(field, value);
    }

    fn record_i64(&mut self, field: &field::Field, value: i64) {
        if self.remove_keys.contains(field.name()) {
            return;
        }
        self.visitor.record_i64(field, value);
    }

    fn record_u64(&mut self, field: &field::Field, value: u64) {
        if self.remove_keys.contains(field.name()) {
            return;
        }
        self.visitor.record_u64(field, value);
    }

    fn record_i128(&mut self, field: &field::Field, value: i128) {
        if self.remove_keys.contains(field.name()) {
            return;
        }
        self.visitor.record_i128(field, value);
    }

    fn record_u128(&mut self, field: &field::Field, value: u128) {
        if self.remove_keys.contains(field.name()) {
            return;
        }
        self.visitor.record_u128(field, value);
    }

    fn record_bool(&mut self, field: &field::Field, value: bool) {
        if self.remove_keys.contains(field.name()) {
            return;
        }
        self.visitor.record_bool(field, value);
    }

    fn record_str(&mut self, field: &field::Field, value: &str) {
        if self.remove_keys.contains(field.name()) {
            return;
        }
        self.visitor.record_str(field, value);
    }

    fn record_bytes(&mut self, field: &field::Field, value: &[u8]) {
        if self.remove_keys.contains(field.name()) {
            return;
        }
        self.visitor.record_bytes(field, value);
    }

    fn record_error(&mut self, field: &field::Field, value: &(dyn std::error::Error + 'static)) {
        if self.remove_keys.contains(field.name()) {
            return;
        }
        self.visitor.record_error(field, value);
    }
}
