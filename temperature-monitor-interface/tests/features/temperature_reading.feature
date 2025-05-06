Feature: Temperature in Fahrenheit or Celsius
    Scenario: The temperature can be read in Celsius from Fahrenheit.
        Given an outside temperature of 75.2 Fahrenheit,
        When the temperature is read in Celsius,
        Then the temperature should be 24 Celsius.

    Scenario: The temperature can be read in Fahrenheit from Fahrenheit.
        Given an outside temperature of 75.2 Fahrenheit,
        When the temperature is read in Fahrenheit,
        Then the temperature should be 75.2 Fahrenheit.

    Scenario: The temperature can be read in Fahrenheit from Celsius.
        Given an outside temperature of 25.3 Celsius,
        When the temperature is read in Fahrenheit,
        Then the temperature should be 77.54 Fahrenheit.

    Scenario: The temperature can be read in Celsius from Celsius.
        Given an outside temperature of 24 Celsius,
        When the temperature is read in Celsius,
        Then the temperature should be 24 Celsius.
