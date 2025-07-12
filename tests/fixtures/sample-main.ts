import App from './sample-app';
import { User } from './sample-utils';

class Main {
    private app: App;

    constructor() {
        this.app = new App();
    }

    public run(): void {
        console.log('アプリケーションを開始しています...');

        const users = this.app.getUsers();
        console.log(`登録されたユーザー数: ${users.length}`);

        users.forEach((user: User) => {
            console.log(`ユーザー: ${user.name} (${user.email})`);
        });

        // 特定のユーザーを検索
        const foundUser = this.app.findUser(0);
        if (foundUser) {
            console.log(`見つかったユーザー: ${foundUser.name}`);
        }
    }
}

const main = new Main();
main.run(); 