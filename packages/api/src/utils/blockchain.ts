import debug from 'debug';

const log = debug('brok:utils:blockchain');

export function handleRPCError(error: any) {
  try {
    // Insufficient funds
    if ('error' in error && 'message' in error.error) {
      log(error.error.message)
      return error.error.message
    }
    // // Generic
    // else if (error instanceof Error) {
    //   const message = JSON.parse(error.message);
    //   if ('error' in message && 'reason' in message.error) {
    //     log(message.error.reason)
    //     return message.error.reason;
    //   }
    // }
    // Fallback

      log('Unknown error:', error)
      return `Unknown error: ${error}`

  } catch (error) {
    log('Error parsing error message: ', error);
    return `Error parsing error message: ${error}`
  } 
}
