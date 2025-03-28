// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PurchaseOffering`](crate::operation::purchase_offering::builders::PurchaseOfferingFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`offering_arn(impl Into<String>)`](crate::operation::purchase_offering::builders::PurchaseOfferingFluentBuilder::offering_arn) / [`set_offering_arn(Option<String>)`](crate::operation::purchase_offering::builders::PurchaseOfferingFluentBuilder::set_offering_arn):<br>required: **true**<br><p>The Amazon Resource Name (ARN) of the offering.</p><br>
    ///   - [`reservation_name(impl Into<String>)`](crate::operation::purchase_offering::builders::PurchaseOfferingFluentBuilder::reservation_name) / [`set_reservation_name(Option<String>)`](crate::operation::purchase_offering::builders::PurchaseOfferingFluentBuilder::set_reservation_name):<br>required: **true**<br><p>The name that you want to use for the reservation.</p><br>
    ///   - [`start(impl Into<String>)`](crate::operation::purchase_offering::builders::PurchaseOfferingFluentBuilder::start) / [`set_start(Option<String>)`](crate::operation::purchase_offering::builders::PurchaseOfferingFluentBuilder::set_start):<br>required: **true**<br><p>The date and time that you want the reservation to begin, in Coordinated Universal Time (UTC).</p> <p>You can specify any date and time between 12:00am on the first day of the current month to the current time on today's date, inclusive. Specify the start in a 24-hour notation. Use the following format: <code>YYYY-MM-DDTHH:mm:SSZ</code>, where <code>T</code> and <code>Z</code> are literal characters. For example, to specify 11:30pm on March 5, 2020, enter <code>2020-03-05T23:30:00Z</code>.</p><br>
    /// - On success, responds with [`PurchaseOfferingOutput`](crate::operation::purchase_offering::PurchaseOfferingOutput) with field(s):
    ///   - [`reservation(Option<Reservation>)`](crate::operation::purchase_offering::PurchaseOfferingOutput::reservation): <p>The details of the reservation that you just created when you purchased the offering.</p>
    /// - On failure, responds with [`SdkError<PurchaseOfferingError>`](crate::operation::purchase_offering::PurchaseOfferingError)
    pub fn purchase_offering(&self) -> crate::operation::purchase_offering::builders::PurchaseOfferingFluentBuilder {
        crate::operation::purchase_offering::builders::PurchaseOfferingFluentBuilder::new(self.handle.clone())
    }
}
