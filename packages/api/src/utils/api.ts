import { NextApiResponse, NextApiRequest } from "next";
import { ApiError } from "next/dist/server/api-utils";

export function ErrorResponse(error: unknown, log: debug.Debugger, res: NextApiResponse) {
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

export function ApiRequestLogger(req: NextApiRequest, log: debug.Debugger) {
  switch (req.method) {
    case 'GET':
      if(req.query) {
        log(`HTTP ${req.method} ${req.url}\nrequest query:`, req.query)
      } else {
        log(`HTTP ${req.method} ${req.url}`)
      }
      break;
    case 'POST':
      if(req.body) {
        log(`HTTP ${req.method} ${req.url}\nrequest body:`, req.body)
      } else {
        log(`HTTP ${req.method} ${req.url}`)
      }
      break;
    default:
      break;
  }
}