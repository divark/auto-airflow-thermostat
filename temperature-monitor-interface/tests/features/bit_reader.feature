Feature: Bit Reading
    Scenario: The number of 1s should be counted in some binary number.
        Given the number 11,
        When the number is read for 4 bits,
        Then there should be 3 1s found.

    Scenario: The number of 0s should be counted in some binary number.
        Given the number 9,
        When the number is read for 4 bits,
        Then there should be 2 0s found.
