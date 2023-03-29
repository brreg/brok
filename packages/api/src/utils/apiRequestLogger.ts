import { NextApiRequest } from "next";

export default function ApiRequestLogger(req: NextApiRequest, log: debug.Debugger) {
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