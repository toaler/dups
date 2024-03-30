import DeleteIcon from '@mui/icons-material/Delete';
import IconButton from '@mui/material/IconButton'; // Assuming you use MUI IconButton for better click handling

function StagingTab({ actions, setActions }) {
    // Function to handle deletion of an item from the actions array
    const handleDelete = (indexToDelete) => {
        setActions(currentActions => currentActions.filter((_, index) => index !== indexToDelete));
    };

    return (
        <div>
            <table>
                <thead>
                <tr>
                    <th style={{textAlign: "left"}}>Remove</th>
                    <th style={{textAlign: "left"}}>Action</th>
                    <th style={{textAlign: "left"}}>Resource</th>
                    <th style={{textAlign: "right"}}>Bytes</th>
                </tr>
                </thead>
                <tbody>
                {actions.map((actionObj, index) => (
                    <tr key={index}>
                        <td>
                            <DeleteIcon onClick={() => handleDelete(index)} style={{padding: 0}}/>
                        </td>
                        <td>{actionObj.action}</td>
                        <td>{actionObj.path}</td>
                        <td style={{textAlign: "right"}}>{actionObj.bytes.toLocaleString("en-US")}</td>
                    </tr>
                ))}
                </tbody>
            </table>
        </div>
    );
}

export default StagingTab;