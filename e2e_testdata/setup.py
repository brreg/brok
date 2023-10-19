import lib
import requests

NO_OF_COMPANIES = 10
NO_OF_SECURITY_HOLDERS = 1000
NO_OF_SECURITY_HOLDERS_PER_COMPANY = 20

companies = lib.create_companies(NO_OF_COMPANIES)
security_holders = lib.create_all_securityholders(NO_OF_SECURITY_HOLDERS)
sc = lib.create_wallets_holders_all_companies(security_holders, NO_OF_SECURITY_HOLDERS_PER_COMPANY, companies)
lib.initate_security_holdings(sc)