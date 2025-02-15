# Setup

Create a config.toml file in the root directory with the following content:

```toml
# Cash buffer ratio as a percentage of the initial savings at start of pension
cash_buffer_ratio = 0.0
# Netto monthly withdrawal during pension
netto_monthly_withdrawal = 0.0
# Monthly saving during working years per month
monthly_saving = 0.0
# Interest rate during working years (as a percentage) is kept in pension but reduced through cash buffer
interest_rate_working = 0.0
# Initial savings at start of simulation meaning today
initial_savings = 0.0
# Number of years the person works from now until pension
working_years = 0
# Number of years the person is in pension
pension_years = 0
```
