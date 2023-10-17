# BRØK Test data generator

Simple test data generator.

To run it, type ```python setup.py``` from the folder ```e2e_testdata```.

The test data generator will create a number of companies, a population (this includes both persons and companies) of security holders and from this population
a gives number of security holders per company where each security holder has an arbitrary number of securities between 100 and 1000.

The population of security holders has statistically 80% persons and 20% companies. 

The system can be configured by editing following variables in the ```setup.py``` file:

```NO_OF_COMPANIES``` - set the number of companies to create.

```NO_OF_SECURITY_HOLDERS``` - the number of persons and companies from which to create actual security holders. The maximum is presently 120 000.

```NO_OF_SECURITY_HOLDERS_PER_COMPANY``` - the number of security holders per company.


