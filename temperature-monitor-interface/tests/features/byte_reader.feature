Feature: Byte Reader
    Scenario: A Byte can be read in little-endian order.
        Given a Bit Reader reading the number 11,
        And a Byte Reader that takes the Bit Reader,
        When a byte is read in little-endian,
        Then the result should be 11.
