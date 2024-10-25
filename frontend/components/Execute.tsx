import Borrow from "./Borrow";
import CreateAccount from "./CreateAccount";
import Repay from "./Repay";

function Execute() {
    return (
        <div className="flex items-center justify-center min-h-screen">
            <div className="space-y-4 p-6 bg-gray-100 rounded-md shadow-lg">
                <CreateAccount />
                <Borrow />
                <Repay />
            </div>
        </div>
    );
}

export default Execute;
