import UserService, { User, DEFAULT_USER, createUser } from './sample-utils';

export class App {
    private userService: UserService;

    constructor() {
        this.userService = new UserService();
        this.initializeApp();
    }

    private initializeApp(): void {
        // デフォルトユーザーを追加
        this.userService.addUser(DEFAULT_USER);

        // サンプルユーザーを作成
        const sampleUser = createUser('John Doe', 'john@example.com');
        this.userService.addUser(sampleUser);
    }

    public getUsers(): User[] {
        return this.userService.getAllUsers();
    }

    public findUser(id: number): User | undefined {
        return this.userService.getUser(id);
    }
}

export default App; 