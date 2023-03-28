import { CapTableRegistry__factory, CapTable__factory } from "@brok/captable";
import { ethers } from "ethers";
import type { NextApiRequest, NextApiResponse } from "next";
import { CONTRACT_ADDRESSES, GET_PROVIDER, SPEND_KEY, WALLET } from "../../../contants";
import { handleRPCError } from "../../../utils/blockchain";
import { getStealthAddress } from "../../../utils/stealth";
import debug from "debug";

type Data = {};
const log = debug("brok:api:shares:issue")

export default async function handler(req: NextApiRequest, res: NextApiResponse<Data>) {
	switch (req.method) {
		case "GET":
      log(`HTTP ${req.method} ${req.url}\nquery:`, req.query)
			// show wallet for user
			res.status(200).json({ message: "ok"});
			res.end();
			break;
		case "POST":
      log(`HTTP ${req.method} ${req.url}\nbody:`, req.body)
      // log("HTTP POST /api/shares/issue\nRequest body:", req)
      try {
        if(!("addressToReceiveTokens" in req.body && "capTableAddress" in req.body && "amount" in req.body)){
          return res.status(400).json({ error: "missing addressToReceiveTokens, capTableAddress or amount in body"})
        }
        const addressToReceiveTokens = req.body.addressToReceiveTokens.toString()
        const capTableAddress = req.body.capTableAddress.toString()
        const amount = parseInt(req.body.amount)
        if (typeof amount !== "number") {
          return res.status(400).json({ error: `amount must be a number, received ${typeof amount}` })
        }

        const wallet = WALLET.connect(GET_PROVIDER())
        const capTable = await new CapTable__factory(wallet).attach(capTableAddress)
        const registry = new CapTableRegistry__factory().attach(CONTRACT_ADDRESSES.CAP_TABLE_REGISTRY).connect(GET_PROVIDER());
				const isVerified = await registry.checkAuthenticatedOnce(addressToReceiveTokens);
        if (!isVerified) {
          return res.status(400).json({ error: `addressToReciveTokens ${addressToReceiveTokens} is not registred in the system, try verifying the address by using /api/shareholder/verify`})
        }
        try {
          log(`trying to issue ${amount} shares to ${addressToReceiveTokens} for CapTable with address ${capTable.address}`)
          const result = await capTable.issue(addressToReceiveTokens, ethers.BigNumber.from(amount),"0x12")
          log(`successfully created transaction to ${addressToReceiveTokens} with hash ${result.hash}`)
          return res.status(200).json({ 
            transaction: {
              created: true,
              completed: false,
              hash: result.hash,
            },
            message: "created a new transaction"
          })
        } catch (error) {
          return res.status(500).json({ 
            error: handleRPCError(error),
            transaction: null,
            message: "could not create transaction, see error for more details"
          })
        }
      } catch (error) {
        return res.status(500).json({ 
          error: error,
          transaction: null,
          message: "could not create transaction, see error for more details"
        })
      }
		default:
			res.setHeader("Allow", ["GET", "POST"]);
			res.status(405).end(`Method ${req.method} Not Allowed`);
	}
}
