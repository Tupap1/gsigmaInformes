use chrono::{Datelike, Local};

pub fn parse_year(date_str: &str) -> Result<i32, String> {
    let date_str = date_str.trim();
    if date_str.is_empty() {
        return Err("Fechas inválidas proporcionadas a getYearTables".to_string());
    }
    let parts: Vec<&str> = date_str.split('-').collect();
    if !parts.is_empty() {
        if let Ok(year) = parts[0].parse::<i32>() {
            return Ok(year);
        }
    }
    Err("Fechas inválidas proporcionadas a getYearTables".to_string())
}

pub fn get_year_tables(
    base_name: &str,
    start_date: &str,
    end_date: &str,
) -> Result<Vec<String>, String> {
    let start_year = parse_year(start_date)?;
    let end_year = parse_year(end_date)?;

    if start_year > end_year {
        return Err("La fecha de inicio no puede ser posterior a la fecha de fin".to_string());
    }

    let current_year = Local::now().year();

    let mut tables = Vec::new();
    for year in start_year..=end_year {
        if year < 2011 || year > 2035 {
            return Err(format!(
                "Año {} fuera de rango de soporte histórico (2011-2035)",
                year
            ));
        }

        if year == current_year {
            tables.push(base_name.to_string());
        } else {
            let suffix = format!("{:02}", year % 100);
            tables.push(format!("{}{}", base_name, suffix));
        }
    }

    tables.dedup();
    Ok(tables)
}

pub fn get_all_year_tables(base_name: &str) -> Vec<String> {
    let current_year = Local::now().year();
    let mut tables = Vec::new();
    for year in 2011..=current_year {
        if year == current_year {
            tables.push(base_name.to_string());
        } else {
            let suffix = format!("{:02}", year % 100);
            tables.push(format!("{}{}", base_name, suffix));
        }
    }
    tables
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_historical_year() {
        let result = get_year_tables("compra", "2024-06-15", "2024-06-15").unwrap();
        assert_eq!(result, vec!["compra24".to_string()]);
    }

    #[test]
    fn test_range_crossing_years() {
        let result = get_year_tables("compra", "2023-12-15", "2024-01-15").unwrap();
        assert_eq!(result, vec!["compra23".to_string(), "compra24".to_string()]);
    }

    #[test]
    fn test_handle_date_objects_as_string() {
        // Simulating ISO DateTime string
        let start = "2023-12-15T00:00:00.000Z";
        let end = "2024-01-15T00:00:00.000Z";
        let result = get_year_tables("compra", start, end).unwrap();
        assert_eq!(result, vec!["compra23".to_string(), "compra24".to_string()]);
    }

    #[test]
    fn test_start_year_after_end_year() {
        let err = get_year_tables("compra", "2024-01-01", "2023-01-01").unwrap_err();
        assert_eq!(err, "La fecha de inicio no puede ser posterior a la fecha de fin");
    }

    #[test]
    fn test_unsupported_year_before_2011() {
        let err = get_year_tables("compra", "2010-12-31", "2011-01-01").unwrap_err();
        assert!(err.contains("Año 2010 fuera de rango de soporte histórico"));
    }

    #[test]
    fn test_unsupported_year_after_2035() {
        let err = get_year_tables("compra", "2035-01-01", "2036-01-01").unwrap_err();
        assert!(err.contains("Año 2036 fuera de rango de soporte histórico"));
    }

    #[test]
    fn test_generate_all_tables() {
        let result = get_all_year_tables("compra");
        assert!(result.len() >= 15);
        assert_eq!(result[0], "compra11".to_string());
        assert_eq!(result[result.len() - 1], "compra".to_string());
    }
}
