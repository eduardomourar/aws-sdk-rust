// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_invoice_summary<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::InvoiceSummary>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::InvoiceSummaryBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "AccountId" => {
                            builder = builder.set_account_id(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "InvoiceId" => {
                            builder = builder.set_invoice_id(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "IssuedDate" => {
                            builder = builder.set_issued_date(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                                tokens.next(),
                                ::aws_smithy_types::date_time::Format::EpochSeconds,
                            )?);
                        }
                        "DueDate" => {
                            builder = builder.set_due_date(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                                tokens.next(),
                                ::aws_smithy_types::date_time::Format::EpochSeconds,
                            )?);
                        }
                        "Entity" => {
                            builder = builder.set_entity(crate::protocol_serde::shape_entity::de_entity(tokens)?);
                        }
                        "BillingPeriod" => {
                            builder = builder.set_billing_period(crate::protocol_serde::shape_billing_period::de_billing_period(tokens)?);
                        }
                        "InvoiceType" => {
                            builder = builder.set_invoice_type(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::InvoiceType::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "OriginalInvoiceId" => {
                            builder = builder.set_original_invoice_id(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "PurchaseOrderNumber" => {
                            builder = builder.set_purchase_order_number(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "BaseCurrencyAmount" => {
                            builder = builder
                                .set_base_currency_amount(crate::protocol_serde::shape_invoice_currency_amount::de_invoice_currency_amount(tokens)?);
                        }
                        "TaxCurrencyAmount" => {
                            builder = builder
                                .set_tax_currency_amount(crate::protocol_serde::shape_invoice_currency_amount::de_invoice_currency_amount(tokens)?);
                        }
                        "PaymentCurrencyAmount" => {
                            builder = builder.set_payment_currency_amount(
                                crate::protocol_serde::shape_invoice_currency_amount::de_invoice_currency_amount(tokens)?,
                            );
                        }
                        _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                    },
                    other => {
                        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                            "expected object key or end object, found: {:?}",
                            other
                        )))
                    }
                }
            }
            Ok(Some(builder.build()))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
