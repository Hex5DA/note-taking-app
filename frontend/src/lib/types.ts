export interface Note {
    id: number,
    title: string,
    content: string,
    starred: boolean
}

export interface OptionalNote {
    title?: string,
    content?: string,
    starred?: boolean
}