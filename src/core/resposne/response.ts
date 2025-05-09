// interface Response<T> {
//     readonly result: string,
//     readonly message?: string
//     readonly data?: T,
// }

export interface Success<T> {
    readonly data: T
}

export interface Failure {
    readonly message: string
}

export type Response<T> = Success<T> | Failure

export function isSuccess<T>(response: Response<T>): response is Success<T> {
    return "data" in response;
}
