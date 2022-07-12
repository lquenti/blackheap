import {useContext} from 'react';
import {Link} from 'react-router-dom'

import ModelContext from '../contexts/ModelContext';

const Dashboard = () => {
  // TODO NULL OPERATOR
  const {jsonStr} = useContext(ModelContext)!;
  return (<div style={{backgroundColor: '#0f0'}}>
    Dashboard: {jsonStr}
    <p>
      <Link to="/">Back to uploader</Link>
    </p >
  </div>);
};

export default Dashboard;
