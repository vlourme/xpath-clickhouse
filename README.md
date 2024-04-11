# XPath UDF for ClickHouse
This is a simple UDF for ClickHouse that allows you to evaluate XPath expressions on XML data.

## Examples
```sql
SELECT xpath('<product><name>Example</name></product>', '//product/name');
-- Output: Example

SELECT xpath('
    <product>
        <name>Example 1</name>
        <name>Example 2</name>
    </product>', '//product/name');
-- Output: Example 1
-- Only the first result is returned

SELECT xpath_to_array('
    <product>
        <name>Example 1</name>
        <name>Example 2</name>
    </product>', '//product/name');
-- Output: ['Example 1', 'Example 2']
```

## Installation
```sh
# Clone the repository
git clone https://github.com/vlourme/xpath-clickhouse.git
cd xpath-clickhouse

# Build the project
cargo build --release

# Copy the releases to ClickHouse's user_scripts
cp target/release/xpath /etc/clickhouse/user_scripts
cp target/release/xpath_to_array /etc/clickhouse/user_scripts

# Copy the UDF declaration to ClickHouse root
cp xpath_function.xml /etc/clickhouse
```

### Side notes
- This project is highly inspired from https://github.com/duyet/clickhouse-udf-rs.