export interface Note {
    id?: number,
    title: string,
    content: string,
    starred: boolean
}

export interface OptionalNote {
    id?: number,
    title?: string,
    content?: string,
    starred?: boolean
}