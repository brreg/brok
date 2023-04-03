import { NextApiResponse } from "next";
import { ApiError } from "next/dist/server/api-utils";

export function errorResponse(error: unknown, log: debug.Debugger, res: NextApiResponse) {
  if (error instanceof ApiError) {
    log(`HTTP Response ${error.statusCode}, ${error.message} ${error}`)
    return res.status(error.statusCode).json({ 
      error: error,
      transaction: null,
      message: error.message
    })
  }
  log(`HTTP Response 500, error ${error}`);
  res.status(500).json({ error })
}