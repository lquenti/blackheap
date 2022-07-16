import Blackheap from "../components/Blackheap";

const NotFound = () => (
  <div>
    <div className="bg-base-100 hero min-h-screen">
      <div className="max-w-lg text-base-content text-center">
        <h1 className="text-8xl mb-5">
          <Blackheap />
        </h1>
        <h2 className="text-2xl font-bold">Website not found!</h2>
        <button className="btn btn-primary btn-block mt-3">
          Back to start
        </button>
      </div>
    </div>
  </div>
);

export default NotFound;
