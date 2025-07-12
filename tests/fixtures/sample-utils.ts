export interface User {
    id: number;
    name: string;
    email: string;
}

export class UserService {
    private users: User[] = [];

    addUser(user: User): void {
        this.users.push(user);
    }

    getUser(id: number): User | undefined {
        return this.users.find(u => u.id === id);
    }

    getAllUsers(): User[] {
        return this.users;
    }
}

export const DEFAULT_USER: User = {
    id: 0,
    name: 'Anonymous',
    email: 'anonymous@example.com'
};

export function createUser(name: string, email: string): User {
    return {
        id: Date.now(),
        name,
        email
    };
}

export default UserService; 