export interface Post {
    title: string;
    description: string;
    content: string;
    createdAt: Date;
}

export interface PostResponse {
    data: PostData[];
    meta: any;
}

export interface PostData {
    id: number;
    attributes: Post;
}