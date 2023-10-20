import lib
import json

NO_OF_COMPANIES = 3
NO_OF_SECURITY_HOLDERS = 10
NO_OF_SECURITY_HOLDERS_PER_COMPANY = 5

companies = lib.create_companies(NO_OF_COMPANIES)

security_holders = lib.create_all_securityholders(NO_OF_SECURITY_HOLDERS)
wallets = lib.create_wallets_holders_all_companies(security_holders, NO_OF_SECURITY_HOLDERS_PER_COMPANY, companies)
sec_holdings = lib.initate_security_holdings(wallets)

with open("companies.json",'w') as outfile:
    outfile.write(json.dumps(companies, indent=2))
with open("security_holders.json",'w') as outfile:
    outfile.write(json.dumps(wallets, indent=2))
with open("initial_holdings.json",'w') as outfile:
    outfile.write(json.dumps(sec_holdings, indent=2))