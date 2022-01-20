export interface Asset {
    icon: string;
    symbol: string;
    balance: number;
}

export interface Transaction {
    title: string;
    status: string;
}

export interface RPC {
    id: number;
    name: string;
}