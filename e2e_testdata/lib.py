import requests
import random
import copy
import logging
import sys

import fnr
import orgnr
import nouns
import adjectives

url = "http://localhost:4000/api/v1/company/"

logger = logging.getLogger('E2E testdata')
logger.setLevel(logging.DEBUG)

handler = logging.StreamHandler(sys.stdout)
handler.setLevel(logging.DEBUG)
formatter = logging.Formatter('%(asctime)s - %(name)s - %(levelname)s - %(message)s')
handler.setFormatter(formatter)
logger.addHandler(handler)

def create_companies(no_of_companies):
    companies = []
    for c in range(0, no_of_companies):
        name = adjectives.adjectives[random.randint(0,len(adjectives.adjectives)-1)] + " " + nouns.nouns[random.randint(0,len(nouns.nouns)-1)]
        orgnr_ = orgnr.orgnr[random.randint(0,len(orgnr.orgnr)-1)]
        c = {'name' : name, 'orgnr' : orgnr_}
        companies.append(c)
        logger.info("Creating company {}, {} created".format(name, orgnr_))
        x = requests.post(url, json = c)
        logger.info(x)
    return companies

def create_all_securityholders(number_of_securityholders):
    if number_of_securityholders > len(fnr.fnr) + len(orgnr.orgnr):
        raise Exception('Cannot create more users than fnr and orgnr in system')
    holderIds = fnr.fnr + orgnr.orgnr
    random.shuffle(holderIds)
    holderIds = holderIds[:number_of_securityholders]
    securityholders = []
    for i in range(0,number_of_securityholders):
        if (len(holderIds[i]) == 11):
            securityholders.append({"OwnerPersonFirstName": nouns.nouns[random.randint(0,len(nouns.nouns)-1)],
                                "OwnerPersonLastName" : adjectives.adjectives[random.randint(0,len(adjectives.adjectives)-1)],
                                "OwnerPersonFnr": holderIds[i]})
        elif (len(holderIds[i]) == 9):
            securityholders.append({"OwnerCompanyName": adjectives.adjectives[random.randint(0,len(adjectives.adjectives)-1)] + " " + nouns.nouns[random.randint(0,len(nouns.nouns)-1)],
                                "OwnerCompanyOrgnr": holderIds[i]})
        else:
            raise Exception('Id is neither fnr nor orgnr, has length ' + str(len(holderIds[i])))
    
    logger.info("Created {} securityholders, {} persons, {} companies".format(str(len(holderIds)), str(len(list(filter(lambda x: len(x) == 11, holderIds)))), str(len(list(filter(lambda x: len(x) == 9, holderIds))))))
    return securityholders

def create_securityholders_for_company(securityholders, orgnr, number_of_securityholders):
    random.shuffle(securityholders)
    securityholders_copy = copy.deepcopy(securityholders)[:number_of_securityholders]
    for s in securityholders_copy:
        s['CapTableOrgnr'] = orgnr

    return securityholders_copy

def create_wallets_holders_all_companies(security_holders, number_of_securityholders_per_company, companies):
    security_holder_per_company = {}
    for c in companies:
        security_holders = create_securityholders_for_company(security_holders,c['orgnr'],number_of_securityholders_per_company)
        url_company = url + c['orgnr'] + "/opprett-lommeboker"
        logger.info("Creating {} wallets for company {}".format(len(security_holders),c['orgnr']))
        x = requests.post(url_company, json = security_holders)
        logger.info(x)
        security_holder_per_company[c['orgnr']] = security_holders
      
    return security_holder_per_company

def initate_security_holdings(companies_with_security_holders):
    for orgnr in companies_with_security_holders:
        recipients = []
        count = []
        for s in companies_with_security_holders[orgnr]:
            if 'OwnerPersonFnr' in s:
                recipients.append(s['OwnerPersonFnr'])
            else:
                recipients.append(s['OwnerCompanyOrgnr'])
            count.append(random.randint(100,1000))
        request_body = {'mottakere' : recipients, 'antall' : count}
       
        url_company = url + orgnr + "/setInitialOwnership"
        logger.info("Setting initial ownership for {} wallets for company {} to random number between 100 and 1000, total supply is {}".format(len(recipients),orgnr,sum(count)))
        x = requests.post(url_company, json = request_body)
        logger.info(x)

